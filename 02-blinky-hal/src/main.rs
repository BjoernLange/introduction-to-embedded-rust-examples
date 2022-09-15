#![no_main]
#![no_std]

mod nrf52840_mdk;

extern crate panic_halt;

use cortex_m_rt::entry;

use nrf52840_hal::gpio::{p0, p1, Level};
use nrf52840_hal::prelude::*;
use nrf52840_hal::timer::Timer;

use nrf52840_pac::Peripherals;

use nrf52840_mdk::Pins;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = Pins::new(p0::Parts::new(peripherals.P0), p1::Parts::new(peripherals.P1));

    // LED pins are active low.
    let mut red_led = pins.red_led.into_push_pull_output(Level::High);
    let mut green_led = pins.green_led.into_push_pull_output(Level::High);
    let mut blue_led = pins.blue_led.into_push_pull_output(Level::High);

    let mut timer = Timer::new(peripherals.TIMER0);

    // Alternately flash the red, green and blue leds.
    loop {
        red_led.set_low().unwrap();
        green_led.set_high().unwrap();
        blue_led.set_high().unwrap();

        timer.delay(250_000); // 250ms
        
        red_led.set_high().unwrap();
        green_led.set_low().unwrap();
        blue_led.set_high().unwrap();
        
        timer.delay(250_000); // 250ms
        
        red_led.set_high().unwrap();
        green_led.set_high().unwrap();
        blue_led.set_low().unwrap();
        
        timer.delay(250_000); // 250ms
    }
}