# blinky-hal

Blinky example using a HAL crate.

## Prerequisites

It is assumed that Rust and python / pip is already installed.

Add the `thumbv7em-none-eabih` target

```
> rustup target add thumbv7em-none-eabih
```

Install pyocd

```
> pip install --user --pre -U pyocd
```

Additionally, install the (system dependent) build tools for `thumbv7em-none-eabih`.

## Building

Run

```
> cargo build
```

## Flashing

Connect the nRF52840-MDK via USB and run

```
> pyocd load -t nrf52 -e chip --format elf target/thumbv7em-none-eabihf/debug/blinky-hal
```

it might be required to create apropriate udev rules for this command to work.

## License notice

This example is based on [Rust for nrf52840-mdk](https://github.com/nrf-rs/nrf52840-mdk-rs) by Anthony James Munns licensed under the MIT license.

Original copyright and permission notice:

```
Copyright (c) 2018 Anthony James Munns

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
```