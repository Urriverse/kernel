use crate::sync::Nutex;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;

pub type MethodId = u64;

extrum! {
    #[derive(Clone, Copy, PartialEq)]
    pub enum DeviceStatus: usize {
        SUCCESS = 0,
        NOT_FOUND = 1,
        INVALID_ARG = 2,
        BUSY = 3,
        IO_ERROR = 4,
        UNSUPPORTED = usize::MAX,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct DeviceResult {
    pub value: usize,
    pub status: DeviceStatus,
}

impl DeviceResult {
    pub const fn new(value: usize, status: DeviceStatus) -> Self {
        Self { value, status }
    }

    pub const fn ok(value: usize) -> Self {
        Self { value, status: DeviceStatus::SUCCESS }
    }

    pub const fn err(status: DeviceStatus) -> Self {
        Self { value: 0, status }
    }

    pub fn as_result(self) -> Result<usize, DeviceStatus> {
        if self.status == DeviceStatus::SUCCESS {
            return Ok(self.value);
        } else {
            return Err(self.status);
        }
    }

    pub fn from_result(res: Result<usize, DeviceStatus>) -> Self {
        match res {
            Ok(value) => Self { value, status: DeviceStatus::SUCCESS },
            Err(status) => Self { value: 0, status },
        }
    }
}

pub type DeviceMethod = extern "C" fn(DeviceId, usize) -> DeviceResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct DeviceId(u32);

impl DeviceId {
    #[inline]
    pub const fn index(self) -> usize { (self.0 & 0x000FFFFF) as usize }
    
    #[inline]
    pub const fn generation(self) -> u16 { (self.0 >> 20) as u16 }
    
    #[inline]
    pub const fn new(index: usize, gen_: u16) -> Self { 
        Self(((gen_ as u32) << 20) | (index as u32)) 
    }
    
    #[inline]
    pub const fn is_null(self) -> bool { self.0 == 0 }
    
    pub const NULL: Self = Self(0);
}

pub struct Device {
    pub id: DeviceId,
    pub name: String,
    pub parent: Option<DeviceId>,
    
    pub driver_data: usize,
    
    pub methods: BTreeMap<MethodId, DeviceMethod>,
}

impl Device {
    pub fn new(name: &str) -> Box<Self> {
        Box::new(Self {
            id: DeviceId::NULL,
            name: String::from(name),
            parent: None,
            driver_data: 0,
            methods: BTreeMap::new(),
        })
    }

    pub fn add_method(&mut self, method_id: MethodId, method: DeviceMethod) {
        self.methods.insert(method_id, method);
    }

    pub fn get_method(&self, method_id: MethodId) -> Option<DeviceMethod> {
        self.methods.get(&method_id).copied()
    }
}

const MAX_DEVICES: usize = 1024;

struct Registry {
    devices: [Option<Box<Device>>; MAX_DEVICES],
    generations: [u16; MAX_DEVICES],
}

impl Registry {
    const fn new() -> Self {
        Self {
            devices: [const { None }; MAX_DEVICES],
            generations: [0; MAX_DEVICES],
        }
    }

    fn register(&mut self, mut device: Box<Device>) -> Option<DeviceId> {
        for (i, slot) in self.devices.iter_mut().enumerate() {
            if slot.is_none() {
                self.generations[i] = self.generations[i].wrapping_add(1);
                let gen_ = self.generations[i];
                let id = DeviceId::new(i, gen_);
                
                device.id = id;
                *slot = Some(device);
                return Some(id);
            }
        }
        None
    }

    fn unregister(&mut self, id: DeviceId) -> bool {
        let idx = id.index();
        if idx >= MAX_DEVICES { return false; }
        
        if let Some(device) = &self.devices[idx] {
            if device.id.generation() == id.generation() {
                self.devices[idx] = None;
                return true;
            }
        }
        false
    }

    fn get_device(&self, id: DeviceId) -> Option<&Device> {
        let idx = id.index();
        if idx >= MAX_DEVICES { return None; }
        
        if let Some(device) = &self.devices[idx] {
            if device.id.generation() == id.generation() {
                return Some(device);
            }
        }
        None
    }
}

static REGISTRY: Nutex<Registry> = Nutex::new(Registry::new());

pub fn init() {
    info!("Device model initialized");
}

pub fn register_device(device: Box<Device>) -> Option<DeviceId> {
    REGISTRY.lock().register(device)
}

pub fn unregister_device(id: DeviceId) -> bool {
    REGISTRY.lock().unregister(id)
}

pub fn set_driver_data(id: DeviceId, data: usize) -> bool {
    let mut guard = REGISTRY.lock();
    if let Some(device) = guard.devices[id.index()].as_mut() {
        if device.id.generation() == id.generation() {
            device.driver_data = data;
            return true;
        }
    }
    false
}

pub fn get_driver_data(id: DeviceId) -> Option<usize> {
    let guard = REGISTRY.lock();
    guard.get_device(id).map(|dev| dev.driver_data)
}

pub fn call_method(id: DeviceId, method_id: MethodId, arg: usize) -> DeviceResult {
    let guard = REGISTRY.lock();
    let device_opt = guard.get_device(id);
    let device;
    let method;
    match device_opt { None => return DeviceResult::new(0, DeviceStatus::NOT_FOUND), Some(x) => device = x, };
    let method_opt = device.get_method(method_id);
    match method_opt { None => return DeviceResult::new(0, DeviceStatus::UNSUPPORTED), Some(x) => method = x, }
    
    drop(guard);
    
    method(id, arg)
}
