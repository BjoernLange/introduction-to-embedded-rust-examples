#![no_std]

pub mod led;

use embedded_hal::blocking::delay::DelayMs;

use crate::led::Led;

pub fn blinky_main<RedLed, GreenLed, BlueLed, Error, Delay>(
    mut red: RedLed,
    mut green: GreenLed,
    mut blue: BlueLed,
    mut delay: Delay,
) -> Result<(), Error>
where
    RedLed: Led,
    GreenLed: Led,
    BlueLed: Led,
    Error: From<RedLed::Error> + From<GreenLed::Error> + From<BlueLed::Error>,
    Delay: DelayMs<u32>,
{
    loop {
        red.off()?;
        green.off()?;
        blue.on()?;

        delay.delay_ms(250);

        red.off()?;
        green.on()?;
        blue.off()?;

        delay.delay_ms(250);

        red.on()?;
        green.off()?;
        blue.off()?;

        delay.delay_ms(250);
    }
}
