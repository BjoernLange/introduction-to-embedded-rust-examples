use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::delay::Ets;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys::EspError;

use blinky::{blinky_main, led};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let red_led = led::active_high(peripherals.pins.gpio3.into_output().unwrap());
    let green_led = led::active_high(peripherals.pins.gpio2.into_output().unwrap());
    let blue_led = led::active_high(peripherals.pins.gpio1.into_output().unwrap());

    let delayer = Ets;

    blinky_main::<_, _, _, EspError, _>(red_led, green_led, blue_led, delayer).unwrap();
}
