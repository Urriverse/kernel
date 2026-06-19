//! Простая, гибкая и безопасная модель устройств и драйверов.
//!
//! Принципы:
//! 1. Используем Box, BTreeMap и String (аллокатор уже инициализирован).
//! 2. Драйвер создает Box<Device>, настраивает его и передает в реестр.
//! 3. DeviceId содержит индекс и поколение для надежной защиты от Use-After-Free.

pub mod fb;

use crate::sync::Nutex;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;

// ============================================================================
// 1. Идентификаторы и типы
// ============================================================================

/// Уникальный 64-битный идентификатор метода (хэш FNV-1a от строки).
pub type MethodId = u64;

/// Коды ошибок устройств.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum DeviceStatus {
    Success,
    NotFound,
    InvalidArg,
    Busy,
    IoError,
    Unsupported,
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
        Self { value, status: DeviceStatus::Success }
    }

    pub const fn err(status: DeviceStatus) -> Self {
        Self { value: 0, status }
    }

    pub fn as_result(self) -> Result<usize, DeviceStatus> {
        if self.status == DeviceStatus::Success {
            return Ok(self.value);
        } else {
            return Err(self.status);
        }
    }

    pub fn from_result(res: Result<usize, DeviceStatus>) -> Self {
        match res {
            Ok(value) => Self { value, status: DeviceStatus::Success },
            Err(status) => Self { value: 0, status },
        }
    }
}

/// Сигнатура метода устройства.
/// `arg` может быть непосредственным значением или указателем на структуру аргументов.
pub type DeviceMethod = extern "C" fn(DeviceId, usize) -> DeviceResult;

/// Безопасный ID устройства: 20 бит индекс + 12 бит поколение.
/// Помещается в u32. Дает до 1 048 576 устройств и 4096 циклов пересоздания на слот.
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

// ============================================================================
// 2. Сущность Устройства
// ============================================================================

/// Представление устройства в реестре.
pub struct Device {
    pub id: DeviceId,
    pub name: String,
    pub parent: Option<DeviceId>,
    
    /// Opaque pointer для приватных данных драйвера (например, указатель на Box<State>).
    pub driver_data: usize,
    
    /// Методы устройства, упорядоченные по MethodId.
    pub methods: BTreeMap<MethodId, DeviceMethod>,
}

impl Device {
    /// Создает новое устройство с заданным именем.
    pub fn new(name: &str) -> Box<Self> {
        Box::new(Self {
            id: DeviceId::NULL, // Будет установлен реестром при регистрации
            name: String::from(name),
            parent: None,
            driver_data: 0,
            methods: BTreeMap::new(),
        })
    }

    /// Добавляет метод к устройству.
    pub fn add_method(&mut self, method_id: MethodId, method: DeviceMethod) {
        self.methods.insert(method_id, method);
    }

    /// Ищет метод по его ID.
    pub fn get_method(&self, method_id: MethodId) -> Option<DeviceMethod> {
        self.methods.get(&method_id).copied()
    }
}

// ============================================================================
// 3. Реестр устройств
// ============================================================================

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

    /// Принимает владение Box<Device>, назначает ID и помещает в реестр.
    fn register(&mut self, mut device: Box<Device>) -> Option<DeviceId> {
        for (i, slot) in self.devices.iter_mut().enumerate() {
            if slot.is_none() {
                self.generations[i] = self.generations[i].wrapping_add(1);
                let gen_ = self.generations[i];
                let id = DeviceId::new(i, gen_);
                
                device.id = id;
                *slot = Some(device); // Передача владения (move semantics)
                return Some(id);
            }
        }
        None // Реестр переполнен
    }

    /// Удаляет устройство по ID. Box уничтожается, память освобождается.
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

    /// Получает ссылку на устройство для вызова метода.
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

// ============================================================================
// 4. Публичный API
// ============================================================================

pub fn init() {
    info!("Device model initialized");
}

/// Зарегистрировать устройство, созданное драйвером.
/// Драйвер передает владение через Box. Реестр становится единственным владельцем.
pub fn register_device(device: Box<Device>) -> Option<DeviceId> {
    REGISTRY.lock().register(device)
}

/// Удалить устройство (например, при hot-unplug).
pub fn unregister_device(id: DeviceId) -> bool {
    REGISTRY.lock().unregister(id)
}

/// Установить приватные данные драйвера.
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

/// Получить приватные данные драйвера.
pub fn get_driver_data(id: DeviceId) -> Option<usize> {
    let guard = REGISTRY.lock();
    guard.get_device(id).map(|dev| dev.driver_data)
}

/// Вызвать метод устройства.
pub fn call_method(id: DeviceId, method_id: MethodId, arg: usize) -> DeviceResult {
    let guard = REGISTRY.lock();
    let device_opt = guard.get_device(id);
    let device;
    let method;
    match device_opt { None => return DeviceResult::new(0, DeviceStatus::NotFound), Some(x) => device = x, };
    let method_opt = device.get_method(method_id);
    match method_opt { None => return DeviceResult::new(0, DeviceStatus::Unsupported), Some(x) => method = x, }
    
    drop(guard);
    
    method(id, arg)
}
