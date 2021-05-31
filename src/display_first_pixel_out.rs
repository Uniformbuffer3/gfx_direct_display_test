use gfx_hal as hal;
use hal::prelude::*;
extern crate gfx_backend_vulkan as back;

pub fn test_display_first_pixel_out(){
    env_logger::init();
    let instance = back::Instance::create("gfx-rs quad", 1).expect("Failed to create an instance!");

    let adapter = {
        let mut adapters = instance.enumerate_adapters();
        for adapter in &adapters {
            println!("{:?}", adapter.info);
        }
        adapters.remove(0)
    };

    // Build a new device and associated command queues
    let family = adapter
        .queue_families
        .iter()
        .next()
        .expect("No queue family supports presentation");

    let physical_device = &adapter.physical_device;
    let sparsely_bound = physical_device
        .features()
        .contains(hal::Features::SPARSE_BINDING | hal::Features::SPARSE_RESIDENCY_IMAGE_2D);
    let mut gpu = unsafe {
        physical_device
            .open(
                &[(family, &[1.0])],
                if sparsely_bound {
                    hal::Features::SPARSE_BINDING | hal::Features::SPARSE_RESIDENCY_IMAGE_2D
                } else {
                    hal::Features::empty()
                },
            )
            .unwrap()
    };
    let _queue_group = gpu.queue_groups.pop().unwrap();
    let device = gpu.device;


    let mut fence = device.create_fence(false).unwrap();

    println!("Enumerating displays");
    let displays = unsafe {adapter.physical_device.enumerate_available_displays().unwrap()};
    let display = &displays[0];
    println!("Used display: {:#?}",display);

    let event = hal::display::DisplayEvent::FirstPixelOut;
    println!("Registering event listener");
    match unsafe{device.register_display_event(&display,&event,&mut fence)}{
        Ok(_)=>(),
        Err(err)=>{panic!("Failed to register hotplug event listener: {:#?}",err);}
    }

    loop {
        match unsafe {device.wait_for_fence(&fence,1_000_000_000)}{
            Ok(signaled)=>{if signaled {println!("First pixel out detected!");break;}}
            Err(err)=>println!("Error while waiting for fence: {:#?}",err)
        }
    }
    unsafe{device.destroy_fence(fence);}
}
