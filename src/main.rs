use std::time::Duration;

use embedded_hal::digital::v2::ToggleableOutputPin;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_sys as _;
use esp_idf_hal::gpio::Gpio2;
use esp_idf_hal::gpio::Output;

fn main(){
    esp_idf_sys::link_patches();

    let peripherals: Peripherals = Peripherals::take().unwrap();
    let mut led: Gpio2<Output> = peripherals.pins.gpio2.into_output().unwrap();

    loop{
        println!("toggle");
        led.toggle().unwrap();
        std::thread::sleep(Duration::from_millis(100));
    }
}
