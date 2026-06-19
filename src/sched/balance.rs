// src/sched/balance.rs
use super::rq::RUNQUEUES;

const BALANCE_INTERVAL_MS: u64 = 100;
static mut LAST_BALANCE: u64 = 0;

pub fn check_balance(current_tick: u64) {
    unsafe {
        if current_tick - LAST_BALANCE < BALANCE_INTERVAL_MS {
            return;
        }
        LAST_BALANCE = current_tick;
    }
    
    let num_cpus = crate::arch::num_cpus();
    let mut loads = [0u64; crate::arch::MAX_CPUS];
    let mut total_load = 0u64;
    
    // Collect loads from all CPUs
    for cpu in 0..num_cpus {
        let rq = RUNQUEUES[cpu].lock();
        loads[cpu] = rq.load();
        total_load += loads[cpu];
        drop(rq);
    }
    
    if num_cpus == 0 || total_load == 0 {
        return;
    }
    
    let avg_load = total_load / num_cpus as u64;
    
    // Find most loaded and least loaded CPUs
    let mut max_cpu = 0;
    let mut min_cpu = 0;
    
    for cpu in 0..num_cpus {
        if loads[cpu] > loads[max_cpu] {
            max_cpu = cpu;
        }
        if loads[cpu] < loads[min_cpu] {
            min_cpu = cpu;
        }
    }
    
    // Migrate if imbalance > 25%
    if loads[max_cpu] > avg_load + (avg_load / 4) && loads[min_cpu] < avg_load {
        migrate_task(max_cpu, min_cpu);
    }
}

fn migrate_task(from_cpu: usize, to_cpu: usize) {
    let mut from_rq = RUNQUEUES[from_cpu].lock();
    let mut to_rq = RUNQUEUES[to_cpu].lock();
    
    // Find a task to migrate (not current, not idle)
    if let Some(task) = from_rq.pick_next() {
        if let Some(mut migrated) = from_rq.remove(task.id) {
            migrated.cpu_affinity = Some(to_cpu);
            to_rq.insert(migrated);
            crate::debug!("Migrated task {} from CPU {} to CPU {}", task.id.0, from_cpu, to_cpu);
        }
    }
}
