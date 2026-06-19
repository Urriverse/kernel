#[cfg(target_arch = "x86_64")]
mod amd64;

#[cfg(target_arch = "x86_64")]
pub use amd64::*;

pub fn blocking_sleep(s: f32) {
    while get_time_from_boot_s() < get_time_from_boot_s() + s {
        core::hint::spin_loop();
    }
}
