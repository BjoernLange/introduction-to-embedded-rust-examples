#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;

use nrf52840_pac::P0;

const RED_PIN_INDEX: usize = 23;
const GREEN_PIN_INDEX: usize = 22;
const BLUE_PIN_INDEX: usize = 24;

#[entry]
fn main() -> ! {
    // LED pins are active low.
    // Initialize red LED to output with initial value high.
    unsafe {
        (*P0::ptr()).outset.write(|w| w.bits(1u32 << RED_PIN_INDEX));
    }
    unsafe { &(*P0::ptr()).pin_cnf[RED_PIN_INDEX] }.write(|w| {
        w.dir().output();
        w.input().disconnect();
        w.pull().disabled();
        w.drive().s0s1();
        w.sense().disabled();
        w
    });

    // Initialize green LED to output with initial value high.
    unsafe {
        (*P0::ptr())
            .outset
            .write(|w| w.bits(1u32 << GREEN_PIN_INDEX));
    }
    unsafe { &(*P0::ptr()).pin_cnf[GREEN_PIN_INDEX] }.write(|w| {
        w.dir().output();
        w.input().disconnect();
        w.pull().disabled();
        w.drive().s0s1();
        w.sense().disabled();
        w
    });

    // Initialize blue LED to output with initial value high.
    unsafe {
        (*P0::ptr())
            .outset
            .write(|w| w.bits(1u32 << BLUE_PIN_INDEX));
    }
    unsafe { &(*P0::ptr()).pin_cnf[BLUE_PIN_INDEX] }.write(|w| {
        w.dir().output();
        w.input().disconnect();
        w.pull().disabled();
        w.drive().s0s1();
        w.sense().disabled();
        w
    });

    // Alternately flash the red, green and blue leds.
    loop {
        unsafe {
            (*P0::ptr()).outclr.write(|w| w.bits(1u32 << RED_PIN_INDEX));
        }
        unsafe {
            (*P0::ptr())
                .outset
                .write(|w| w.bits(1u32 << GREEN_PIN_INDEX));
        }
        unsafe {
            (*P0::ptr())
                .outset
                .write(|w| w.bits(1u32 << BLUE_PIN_INDEX));
        }

        unsafe {
            (*P0::ptr()).outset.write(|w| w.bits(1u32 << RED_PIN_INDEX));
        }
        unsafe {
            (*P0::ptr())
                .outclr
                .write(|w| w.bits(1u32 << GREEN_PIN_INDEX));
        }
        unsafe {
            (*P0::ptr())
                .outset
                .write(|w| w.bits(1u32 << BLUE_PIN_INDEX));
        }

        unsafe {
            (*P0::ptr()).outset.write(|w| w.bits(1u32 << RED_PIN_INDEX));
        }
        unsafe {
            (*P0::ptr())
                .outset
                .write(|w| w.bits(1u32 << GREEN_PIN_INDEX));
        }
        unsafe {
            (*P0::ptr())
                .outclr
                .write(|w| w.bits(1u32 << BLUE_PIN_INDEX));
        }
    }
}
