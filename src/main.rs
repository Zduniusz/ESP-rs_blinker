use std::thread::sleep;
use std::time::Duration;
use esp_idf_hal::prelude::Peripherals;
use embedded_hal::digital::blocking::ToggleableOutputPin;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led = peripherals.pins.gpio2.into_output().unwrap();

    let mut index: f32 = 0.01;

    loop {
        index = if index >= 0.50 {0.01} else {index + 0.01};
        let sleep_time = (1.5 / index + 1.0) * 10.0;

        led.toggle();

        println!("Sleeping:  {}", sleep_time);

        sleep(Duration::from_millis( sleep_time as u64));
    }
}