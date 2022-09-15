use embedded_hal::digital::v2::OutputPin;

pub trait Led {
    type Error;

    fn on(&mut self) -> Result<(), Self::Error>;
    fn off(&mut self) -> Result<(), Self::Error>;
}

pub struct ActiveLowLed<Pin>
where
    Pin: OutputPin,
{
    pin: Pin,
}

pub fn active_low<P>(pin: P) -> ActiveLowLed<P>
where
    P: OutputPin,
{
    ActiveLowLed { pin }
}

impl<Pin> Led for ActiveLowLed<Pin>
where
    Pin: OutputPin,
{
    type Error = Pin::Error;

    fn on(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }

    fn off(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }
}

pub struct ActiveHighLed<Pin>
where
    Pin: OutputPin,
{
    pin: Pin,
}

pub fn active_high<P>(pin: P) -> ActiveHighLed<P>
where
    P: OutputPin,
{
    ActiveHighLed { pin }
}

impl<Pin> Led for ActiveHighLed<Pin>
where
    Pin: OutputPin,
{
    type Error = Pin::Error;

    fn on(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }

    fn off(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }
}
