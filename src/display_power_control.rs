use gfx_hal as hal;
use hal::{display,prelude::*};
extern crate gfx_backend_vulkan as back;

pub fn test_display_power_off_on(){
    let instance = back::Instance::create("gfx-rs quad", 1).expect("Failed to create an instance!");

    let adapter = {
        let mut adapters = instance.enumerate_adapters();
        for adapter in &adapters {
            println!("{:?}", adapter.info);
        }
        adapters.remove(0)
    };

    let displays = unsafe{adapter.physical_device.enumerate_available_displays().expect("Failed to enumerate displays")};
    if displays.len() == 0 {panic!("No display is available to create a surface. This means no display is connected or the connected ones are already managed by some other programs. If that is the case, try running the program from a tty terminal.");}

    //Get the first available display
    let display = &displays[0];
    println!("Display: {:#?}",&display);

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

    println!("Powering off display");
    unsafe{device.set_display_power_state(&display,&display::PowerState::Off)}.unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));

    println!("Powering on display");
    unsafe{device.set_display_power_state(&display,&display::PowerState::On)}.unwrap();
}
