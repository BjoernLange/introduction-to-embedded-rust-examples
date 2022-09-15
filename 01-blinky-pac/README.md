# blinky-pac

Blinky example using a PAC crate.

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
> pyocd load -t nrf52 -e chip --format elf target/thumbv7em-none-eabihf/debug/blinky-pac
```

it might be required to create apropriate udev rules for this command to work.
