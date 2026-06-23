//! # Event Bus (EBus) – Global + Per‑CPU Queues with Time‑Limited Workers
//!
//! - Default events go to a global lock‑free queue, processed by an unpinned worker.
//! - Affinity events go to a specific CPU’s queue, processed by that CPU’s pinned worker.
//! - Workers yield after 10 ms of continuous event processing to avoid starving other tasks.
//! - Slow callbacks (>5 ms) trigger a warning.

use crate::arch;
use crate::sched;
use crate::sync::{Nutex, RwLock};
use crate::sched::wq::WaitQueue;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

// ============================================================================
// TYPES
// ============================================================================

pub type EventId = u64;
pub type EventCallback = fn(EventId, usize);

#[derive(Debug, Clone, Copy)]
pub struct Event {
    pub id: EventId,
    pub data: usize,
}

struct Subscriber {
    callback: EventCallback,
}

// ============================================================================
// LOCK‑FREE MPSC QUEUE (Michael‑Scott)
// ============================================================================

struct Node<T> {
    value: Option<T>,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Box<Self> {
        Box::new(Self {
            value: Some(value),
            next: AtomicPtr::new(ptr::null_mut()),
        })
    }

    fn sentinel() -> Box<Self> {
        Box::new(Self {
            value: None,
            next: AtomicPtr::new(ptr::null_mut()),
        })
    }
}

pub struct MpscQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

impl<T> MpscQueue<T> {
    pub const fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
            tail: AtomicPtr::new(ptr::null_mut()),
        }
    }

    pub fn init(&self) {
        let sentinel = Box::into_raw(Node::<T>::sentinel());
        self.head.store(sentinel, Ordering::Release);
        self.tail.store(sentinel, Ordering::Release);
    }

    pub fn enqueue(&self, value: T) -> Result<(), ()> {
        let new_node = Box::into_raw(Node::new(value));
        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };
            if next.is_null() {
                if unsafe { (*tail).next.compare_exchange(
                    ptr::null_mut(),
                    new_node,
                    Ordering::Release,
                    Ordering::Relaxed,
                ) }.is_ok() {
                    let _ = self.tail.compare_exchange(
                        tail,
                        new_node,
                        Ordering::Release,
                        Ordering::Relaxed,
                    );
                    return Ok(());
                }
            } else {
                let _ = self.tail.compare_exchange(
                    tail,
                    next,
                    Ordering::Release,
                    Ordering::Relaxed,
                );
            }
        }
    }

    pub fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };

            if head == tail {
                if next.is_null() {
                    return None;
                }
                let _ = self.tail.compare_exchange(
                    tail,
                    next,
                    Ordering::Release,
                    Ordering::Relaxed,
                );
            } else {
                if let Some(value) = unsafe { (*next).value.take() } {
                    if self.head.compare_exchange(
                        head,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed,
                    ).is_ok() {
                        unsafe { drop(Box::from_raw(head)); }
                        return Some(value);
                    }
                } else {
                    return None;
                }
            }
        }
    }
}

unsafe impl<T: Send> Sync for MpscQueue<T> {}
unsafe impl<T: Send> Send for MpscQueue<T> {}

// ============================================================================
// CONSTANTS
// ============================================================================

/// Maximum time (in ms) a worker may spend processing events before yielding.
const MAX_WORKER_TIME_MS: u64 = 10;

/// If a single callback takes longer than this (ms), a warning is logged.
const SLOW_CALLBACK_WARN_MS: u64 = 5;

// ============================================================================
// GLOBAL STATE
// ============================================================================

static SUBSCRIBERS: RwLock<BTreeMap<EventId, Vec<Subscriber>>> =
    RwLock::new(BTreeMap::new());

static GLOBAL_QUEUE: MpscQueue<Event> = MpscQueue::new();
static mut CPU_QUEUES: [MpscQueue<Event>; arch::MAX_CPUS] =
    [const{MpscQueue::new()}; arch::MAX_CPUS];

static GLOBAL_WQ: Nutex<WaitQueue> = Nutex::new(WaitQueue::new());
static mut CPU_WQS: [Nutex<WaitQueue>; arch::MAX_CPUS] =
    [const{Nutex::new(WaitQueue::new())}; arch::MAX_CPUS];

static INITIALIZED: core::sync::atomic::AtomicBool =
    core::sync::atomic::AtomicBool::new(false);

// ============================================================================
// EVENT DISPATCH (with slow‑callback detection)
// ============================================================================

#[inline]
fn dispatch_event(event: Event) {
    let guard = SUBSCRIBERS.read();
    if let Some(subs) = guard.get(&event.id) {
        for sub in subs {
            let start = arch::get_time_from_boot();
            (sub.callback)(event.id, event.data);
            let elapsed = arch::get_time_from_boot() - start;
            if elapsed > SLOW_CALLBACK_WARN_MS {
                crate::warn!(
                    "EBus: slow callback for event {:#x} took {} ms (limit {} ms)",
                    event.id, elapsed, SLOW_CALLBACK_WARN_MS
                );
            }
        }
    }
}

// ============================================================================
// WORKER TASKS (with time‑limited processing)
// ============================================================================

fn global_worker() {
    loop {
        sched::sleep(&GLOBAL_WQ);

        let mut start_time = arch::get_time_from_boot();
        while let Some(event) = GLOBAL_QUEUE.dequeue() {
            dispatch_event(event);

            let now = arch::get_time_from_boot();
            if now - start_time > MAX_WORKER_TIME_MS {
                // Yield the CPU to allow other tasks to run.
                sched::yield_now();
                // Reset the timer for the next batch.
                start_time = arch::get_time_from_boot();
            }
        }
    }
}

fn cpu_worker() {
    let cpu = arch::current_cpu();
    loop {
        unsafe { sched::sleep(&CPU_WQS[cpu]); }

        let mut start_time = arch::get_time_from_boot();
        while let Some(event) = unsafe { CPU_QUEUES[cpu].dequeue() } {
            dispatch_event(event);

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
        return;
    }

    GLOBAL_QUEUE.init();

    let num_cpus = arch::num_cpus();
    for cpu in 0..num_cpus {
        unsafe {
            CPU_QUEUES[cpu].init();
        }
        sched::spawn_kernel_task(
            cpu_worker,
            sched::task::Priority(0),
            "ebus_cpu_worker",
            None,
            Some(cpu), // pinned to this CPU
        );
    }

    // Global worker – unpinned (scheduler decides where to run)
    sched::spawn_kernel_task(
        global_worker,
        sched::task::Priority(0),
        "ebus_global_worker",
        None,
        None,
    );
}

#[allow(dead_code)]
pub fn subscribe(event_id: EventId, callback: EventCallback) -> Result<(), ()> {
    let mut guard = SUBSCRIBERS.write();
    guard.entry(event_id).or_insert_with(Vec::new).push(Subscriber { callback });
    Ok(())
}

#[allow(dead_code)]
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

/// Publish an event.
///
/// * `affinity: None` – enqueued to the global queue.
/// * `affinity: Some(cpu)` – enqueued to that CPU’s local queue.
#[allow(dead_code)]
pub fn publish(event_id: EventId, data: usize, affinity: Option<usize>) -> Result<(), ()> {
    let event = Event { id: event_id, data };

    match affinity {
        None => {
            GLOBAL_QUEUE.enqueue(event).map_err(|_| ())?;
            sched::wakeup(&GLOBAL_WQ);
        }
        Some(cpu) => {
            if cpu >= arch::MAX_CPUS {
                return Err(());
            }
            unsafe {
                CPU_QUEUES[cpu].enqueue(event).map_err(|_| ())?;
                sched::wakeup(&CPU_WQS[cpu]);
            }
        }
    }
    Ok(())
}
