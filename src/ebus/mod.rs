//! # Event Bus (EBus) – Global + Per‑CPU Queues with Isolated Subscriber Threads
//!
//! - Default events go to a global bounded queue, processed by an unpinned worker.
//! - Affinity events go to a specific CPU’s bounded queue, processed by that CPU’s pinned worker.
//! - Workers spawn an isolated kernel thread for EACH subscriber callback via `spawn_closure_task`.
//! - This guarantees the worker thread NEVER blocks, even if a subscriber infinite-loops.
//! - Slow callbacks (>5 ms) trigger a warning before the thread exits.

use crate::arch;
use crate::sched;
use crate::sync::{Nutex, RwLock};
use crate::sched::wq::WaitQueue;
use alloc::borrow::ToOwned as _;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;
use core::hint::unlikely;
use core::mem::MaybeUninit;
use core::sync::atomic::Ordering;

pub use ketypes::ebus::{KeEventCallback as EventCallback, KeEventId as EventId};

#[derive(Debug, Clone, Copy)]
pub struct Event {
    pub id: EventId,
    pub data: usize,
}

struct Subscriber {
    callback: EventCallback,
}

// ============================================================================
// BOUNDED QUEUE
// ============================================================================
const QUEUE_CAPACITY: usize = 65536;

struct BoundedQueue {
    inner: Nutex<VecDeque<Event>>,
}

impl BoundedQueue {
    pub fn new() -> Self {
        Self {
            inner: Nutex::new(VecDeque::new()),
        }
    }

    pub fn enqueue(&self, event: Event) -> Result<(), ()> {
        let mut q = self.inner.lock();
        if unlikely(q.len() >= QUEUE_CAPACITY) {
            crate::warn!("EBus: Queue full, dropping event {:#x}", event.id);
            return Err(());
        }
        q.push_back(event);
        Ok(())
    }

    pub fn dequeue(&self) -> Option<Event> {
        let mut q = self.inner.lock();
        q.pop_front()
    }
}

// ============================================================================
// CONSTANTS
// ============================================================================
const MAX_WORKER_TIME_MS: u64 = 10;
const SLOW_CALLBACK_WARN_MS: u64 = 5;

// ============================================================================
// GLOBAL STATE
// ============================================================================
static SUBSCRIBERS: RwLock<BTreeMap<EventId, Vec<Subscriber>>> =
    RwLock::new(BTreeMap::new());

lazy_static! {
    static ref GLOBAL_QUEUE: BoundedQueue = BoundedQueue::new();
}

static mut CPU_QUEUES: [MaybeUninit<BoundedQueue>; arch::MAX_CPUS] =
    [const { MaybeUninit::uninit() }; arch::MAX_CPUS];

static GLOBAL_WQ: Nutex<WaitQueue> = Nutex::new(WaitQueue::new());
static mut CPU_WQS: [MaybeUninit<Nutex<WaitQueue>>; arch::MAX_CPUS] =
    [const { MaybeUninit::uninit() }; arch::MAX_CPUS];

static INITIALIZED: core::sync::atomic::AtomicBool =
    core::sync::atomic::AtomicBool::new(false);

// ============================================================================
// EVENT DISPATCH (Spawns isolated threads via Closures)
// ============================================================================
#[inline]
fn dispatch_event(event: Event, affinity: Option<usize>) {
    // 1. Snapshot callbacks under the read lock
    let callbacks: Vec<EventCallback> = {
        let guard = SUBSCRIBERS.read();
        if let Some(subs) = guard.get(&event.id) {
            subs.iter().map(|s| s.callback).collect()
        } else {
            return;
        }
    }; // Read lock is safely dropped here

    // 2. Spawn an isolated thread for each callback
    for callback in callbacks {
        let start_time = arch::get_time_from_boot();
        
        // The closure captures `callback`, `event`, and `start_time` by value.
        sched::spawn_closure(
            move || {
                // Execute the subscriber callback
                callback(event.id, event.data);
                
                // Check execution time
                let elapsed = arch::get_time_from_boot() - start_time;
                if elapsed > SLOW_CALLBACK_WARN_MS {
                    crate::warn!(
                        "EBus: subscriber thread for event {:#x} took {} ms (limit {} ms)",
                        event.id, elapsed, SLOW_CALLBACK_WARN_MS
                    );
                }
            },
            sched::task::Priority(0),
            "ebus_sub".to_owned(),
            affinity, // Inherit affinity so it stays on the correct CPU if applicable
        );
    }
}

// ============================================================================
// WORKER TASKS (Pure Dispatchers)
// ============================================================================
fn global_worker() {
    info!("Started global worker");
    loop {
        sched::sleep(&GLOBAL_WQ);
        let mut start_time = arch::get_time_from_boot();
        while let Some(event) = GLOBAL_QUEUE.dequeue() {
            dispatch_event(event, None);
            let now = arch::get_time_from_boot();
            if now - start_time > MAX_WORKER_TIME_MS {
                sched::yield_now();
                start_time = arch::get_time_from_boot();
            }
        }
    }
}

fn cpu_worker() {
    let cpu = arch::current_cpu();
    
    let (queue, wq) = unsafe {
        (
            &*CPU_QUEUES[cpu].as_ptr(),
            &*CPU_WQS[cpu].as_ptr(),
        )
    };

    info!("Started per-CPU worker");

    loop {
        sched::sleep(wq);
        let mut start_time = arch::get_time_from_boot();
        while let Some(event) = queue.dequeue() {
            dispatch_event(event, Some(cpu));
            let now = arch::get_time_from_boot();
            if now - start_time > MAX_WORKER_TIME_MS {
                sched::yield_now();
                start_time = arch::get_time_from_boot();
            }
        }
    }
}

// ============================================================================
// PUBLIC API
// ============================================================================
pub fn init() {
    if INITIALIZED.swap(true, Ordering::AcqRel) {
        error!("Already initialized");
        return;
    }

    let num_cpus = arch::num_cpus();
    for cpu in 0..num_cpus {
        unsafe {
            CPU_QUEUES[cpu].write(BoundedQueue::new());
            CPU_WQS[cpu].write(Nutex::new(WaitQueue::new()));
        }
        
        sched::spawn(
            cpu_worker,
            sched::task::Priority(0),
            "ebus_cpu_worker".to_owned(),
            Some(cpu), 
            false,
        );
    }

    sched::spawn(
        global_worker,
        sched::task::Priority(0),
        "ebus_global_worker".to_owned(),
        None,
        false,
    );

    info!("Initialized");
}

pub fn subscribe(event_id: EventId, callback: EventCallback) -> Result<(), ()> {
    let mut guard = SUBSCRIBERS.write();
    guard.entry(event_id).or_insert_with(Vec::new).push(Subscriber { callback });
    Ok(())
}

pub fn unsubscribe(event_id: EventId, callback: EventCallback) -> Result<(), ()> {
    let mut guard = SUBSCRIBERS.write();
    if let Some(subs) = guard.get_mut(&event_id) {
        subs.retain(|s| s.callback as *const () != callback as *const ());
        if subs.is_empty() {
            guard.remove(&event_id);
        }
        Ok(())
    } else {
        Err(())
    }
}

pub fn publish(event_id: EventId, data: usize, affinity: Option<usize>) -> Result<(), ()> {
    let event = Event { id: event_id, data };
    match affinity {
        None => {
            GLOBAL_QUEUE.enqueue(event)?;
            sched::wakeup(&GLOBAL_WQ);
        }
        Some(cpu) => {
            if cpu >= arch::MAX_CPUS {
                crate::warn!("EBus: Invalid CPU affinity {}", cpu);
                return Err(());
            }
            unsafe {
                let queue = &*CPU_QUEUES[cpu].as_ptr();
                let wq = &*CPU_WQS[cpu].as_ptr();
                queue.enqueue(event)?;
                sched::wakeup(wq);
            }
        }
    }
    Ok(())
}
