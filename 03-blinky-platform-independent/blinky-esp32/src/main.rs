use esp_idf_hal::delay::Ets;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys::EspError;

use blinky::{blinky_main, led};

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let red_led = led::active_high(peripherals.pins.gpio25.into_output()?);
    let green_led = led::active_high(peripherals.pins.gpio26.into_output()?);
    let blue_led = led::active_high(peripherals.pins.gpio27.into_output()?);

    let delayer = Ets;

    blinky_main::<_, _, _, EspError, _>(red_led, green_led, blue_led, delayer)?;

    loop {}
}