pub use crate::dev::*;

#[macro_export]
macro_rules! driver_method {
    (
        $(#[$meta:meta])*
        $name:ident($dev:ident, $args:ident: &$arg_ty:ty): $($body:tt)*
    ) => {
        $(#[$meta])*
        extern "C" fn $name(__raw_dev: $crate::dev::DeviceId, __raw_arg: usize) -> crate::dev::DeviceResult {
            fn ___($dev: $crate::dev::DeviceId, $args: &$arg_ty) -> Result<usize, crate::dev::DeviceStatus> {
                $($body)*
            }
            crate::dev::DeviceResult::from_result(___(__raw_dev, unsafe { &*(__raw_arg as *const $arg_ty) }))
        }
    };
    (
        $(#[$meta:meta])*
        $name:ident($dev:ident): $($body:tt)*
    ) => {
        $(#[$meta])*
        extern "C" fn $name(__raw_dev: $crate::dev::DeviceId, __raw_arg: usize) -> crate::dev::DeviceResult {
            fn ___($dev: $crate::dev::DeviceId) -> Result<usize, crate::dev::DeviceStatus> {
                $($body)*
            }
            crate::dev::DeviceResult::from_result(___(__raw_dev))
        }
    };
}

#[macro_export]
macro_rules! call_dev_method {
    ($dev_id:expr, $iface:expr, $args:expr) => {
        {
            let __args_ref = &($args);
            $crate::dev::call_method($dev_id, $iface, __args_ref as *const _ as usize)
        }
    };
}

#[macro_export]
macro_rules! interface {
    ($s:expr) => {{
        const fn fnv1a64(data: &[u8]) -> u64 {
            const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
            const FNV_PRIME: u64 = 0x100000001b3;
            let mut hash = FNV_OFFSET_BASIS;
            let mut i = 0;
            while i < data.len() {
                hash ^= data[i] as u64;
                hash = hash.wrapping_mul(FNV_PRIME);
                i += 1;
            }
            hash
        }
        fnv1a64($s)
    }};
}
