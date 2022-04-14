use std::thread;
use std::time::Duration;

use embedded_hal::digital::v2::ToggleableOutputPin;


use esp_idf_hal::peripherals::Peripherals;

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led = peripherals.pins.gpio5.into_output().unwrap();

    loop {
        println!("Hello world!");
        led.toggle().unwrap();
        
        // we are using thread::sleep here to make sure the watchdog isn't triggered
        thread::sleep(Duration::from_millis(1000));
    }
}