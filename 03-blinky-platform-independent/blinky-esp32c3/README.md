# blinky-esp32c3

Blinky example using a platform independent implementation from another crate targeting ESP32-C3.

## Prerequisites

It is assumed that Rust is already installed.

For flashing via USB without a debugger install the following `cargo` sub-commands:

```
cargo install ldproxy
cargo install espflash
cargo install espmonitor
```

Also install a nightly toolchain:

```
rustup toolchain install nightly --component rust-src
```

For flashing and debugging via JTAG install the following:

```
pip install esptool
cargo install espup
espup install
```

You will also need Python 2.7.

## Hardware setup

Connect an RGB-LED (or three separate LEDs) to the pins 3 (red), 2 (green) and 1 (blue) using appropriate resistors.
Connect the cathode to GND.

## Building

Run

```
> cargo build
```

## Flashing

### Via USB

Connect the ESP32-C3 via USB and run

```
> espflash /dev/ttyUSBX target/riscv32imc-esp-espidf/debug/blinky-esp32c3
```

where `/dev/ttyUSBX` is the serial port of the ESP32-C3.

### Via JTAG

Connect the ESP32-C3 pins with a modified USB cable to the PC as described in the espressif documentation. Further info on how to do this can be found [here](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/api-guides/jtag-debugging/configure-builtin-jtag.html), [here](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/api-guides/jtag-debugging/index.html#jtag-debugging-configuring-target) and [here](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/api-guides/jtag-debugging/using-debugger.html#command-line).

Run:

```
> . $HOME/export-esp.sh
> $HOME/.espressif/tools/openocd-esp32/v0.11.0-esp32-20211220/openocd-esp32/bin/openocd \
  -f board/esp32c3-builtin.cfg \
  -c “program_esp target/riscv32imc-esp-espidf/debug/blinky-esp32c3.bin 0x10000 verify exit”
```

Make sure that you converted the ELF binary to an image using `esptool`!

## Debugging

### From a terminal

Connect the ESP32-C3 to your computer as described in [Via JTAG](#via-jtag) and run

```
> . $HOME/export-esp.sh
> $HOME/.espressif/tools/openocd-esp32/v0.11.0-esp32-20211220/openocd-esp32/bin/openocd \
  -f board/esp32c3-builtin.cfg
```

Keep the terminal open and from another one run

```
> . $HOME/export-esp.sh
> riscv32-esp-elf-gdb target/riscv32imc-esp-espidf/debug/blinky-esp32c3
(gdb) target remote :3333
...
(gdb) set remote hardware-watchpoint-limit 2
(gdb) monitor reset halt
...
(gdb) flushregs
...
(gdb) break blinky::blinky_main
...
(gdb) continue
...
```

You can now use the GDB session.

### From VSCode

1. Open the `blinky-platform-independent` directory in VSCode so the `.vscode` directory is available on the top level.
2. Install the [Native Debug](https://open-vsx.org/extension/webfreak/debug) VSCode extension.

The project can be build by running the "Build ESP32-C3 (cargo)" task configured in `.vscode/tasks.json`.

A debugging session is started by running the "Debug ESP32-C3 (openocd)" launch configuration configured in `.vscode/launch.json`.

**Note**: The tasks **DO NOT** flash the compiled binary at the moment!
