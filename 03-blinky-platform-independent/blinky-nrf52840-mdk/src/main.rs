#![no_main]
#![no_std]

mod nrf52840_mdk;

extern crate panic_halt;

use cortex_m_rt::entry;

use nrf52840_hal::gpio::{p0, p1, Level};
use nrf52840_hal::timer::Timer;

use nrf52840_pac::Peripherals;

use nrf52840_mdk::Pins;

use void::Void;

use blinky::{blinky_main, led};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = Pins::new(
        p0::Parts::new(peripherals.P0),
        p1::Parts::new(peripherals.P1),
    );

    let red_led = led::active_low(pins.red_led.into_push_pull_output(Level::High));
    let green_led = led::active_low(pins.green_led.into_push_pull_output(Level::High));
    let blue_led = led::active_low(pins.blue_led.into_push_pull_output(Level::High));

    let timer = Timer::new(peripherals.TIMER0);

    blinky_main::<_, _, _, Void, _>(red_led, green_led, blue_led, timer).unwrap();

    loop {}
}
