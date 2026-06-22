use crate::driverkit::*;
use alloc::boxed::Box;

const FB_GET_INFO: MethodId = interface!(b"fb.get_info");
const FB_CLEAR   : MethodId = interface!(b"fb.clear");
const FB_PLOT    : MethodId = interface!(b"fb.plot");

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FbInfo {
    pub address: usize,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PlotArgs {
    pub x: u32,
    pub y: u32,
    pub color: u32,
}

pub struct FbState {
    pub info: FbInfo,
}

driver_method! {
    fb_get_info_method(dev):

    match crate::dev::get_driver_data(dev) {
        Some(x) => Ok(x as usize),
        None => Err(DeviceStatus::NotFound),
    }
}

driver_method! {
    fb_clear_method(dev, color: &usize):

    let state_ptr = fb_get_info_method(dev, 0).as_result()? as *const FbState;

    let state = unsafe { &*state_ptr };

    if state.info.bpp != 32 { return Err(DeviceStatus::Unsupported); }

    let pixels = (state.info.width * state.info.height) as usize;
    let ptr = state.info.address as *mut u32;

    for i in 0..pixels {
        unsafe { core::ptr::write_volatile(ptr.add(i), *color as u32); }
    }

    Ok(pixels)
}

driver_method! {
    fb_plot_method(dev, args: &PlotArgs):

    let state_ptr = fb_get_info_method(dev, 0).as_result()? as *const FbState;

    let state = unsafe { &*state_ptr };

    if args.x >= state.info.width || args.y >= state.info.height {
        return Err(DeviceStatus::InvalidArg);
    }
    if state.info.bpp != 32 { return Err(DeviceStatus::Unsupported); }

    let offset = ((args.y * state.info.pitch) + (args.x * 4)) as usize;
    let ptr = (state.info.address + offset) as *mut u32;
    unsafe { core::ptr::write_volatile(ptr, args.color); }

    Ok(0usize)
}

limine! { FBR <= FramebufferRequest }

pub fn probe() -> Option<DeviceId> {
    info!("Probing Limine Framebuffer...");
    let response = FBR.response()?;
    let fb = response.framebuffers().get(0)?;

    info!("Found {}x{} @ {:#X}", fb.width, fb.height, fb.address() as usize);

    let mut dev = Device::new("fb0");
    
    dev.add_method(FB_GET_INFO, fb_get_info_method);
    dev.add_method(FB_CLEAR, fb_clear_method);
    dev.add_method(FB_PLOT, fb_plot_method);
    
    let state = Box::new(FbState {
        info: FbInfo {
            address: fb.address() as usize,
            width: fb.width as u32,
            height: fb.height as u32,
            pitch: fb.pitch as u32,
            bpp: fb.bpp as u8,
        },
    });
    
    dev.driver_data = Box::into_raw(state) as usize;
    
    let dev_id = crate::dev::register_device(dev)?;
    
    info!("Registered with ID {:?}", dev_id);
    Some(dev_id)
}
