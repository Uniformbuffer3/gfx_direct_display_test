mod display_power_control;
mod display_hotplug;
mod display_first_pixel_out;

fn main() {
    display_first_pixel_out::test_display_first_pixel_out();
    //display_hotplug::test_display_hotplug();
    //display_power_control::test_display_power_off_on();
}
