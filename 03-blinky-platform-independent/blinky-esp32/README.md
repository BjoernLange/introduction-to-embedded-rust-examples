# blinky-esp32

Blinky example using a platform independent implementation from another crate targeting ESP32.

## Prerequisites

It is assumed that Rust and docker is already installed.

For flashing via USB without a debugger install `espflash` (on your system, not in the docker container):

```
cargo install espflash
```

For flashing and debugging install [Espressif IDF extension for VSCode](https://marketplace.visualstudio.com/items?itemName=espressif.esp-idf-extension) and complete the setup.

## Hardware setup

Connect an RGB-LED (or three separate LEDs) to the pins 25 (red), 26 (green) and 27 (blue) using appropriate resistors.
Connect the cathode to GND. 

## Building

Startup the docker container containing a pre-installed environment for building for ESP32 with the project mounted:

```
# Remember: We have a local dependency and thus need to mount the parent folder!
> docker run -v $(pwd)/..:/opt/project -it espressif/idf-rust:esp32_1.63.0.2
```

Inside the container run

```
> rustup default esp
> cd /opt/project/blinky-esp32
> cargo build
```

Optionally, outside the docker container, convert the ELF file to an image for flashing via debugger

```
> . $HOME/esp/esp-idf/export.sh
> esptool.py --chip ESP32 elf2image target/xtensa-esp32-espidf/debug/blinky-esp32
```

## Flashing

### Via USB

Connect the ESP32 via USB and run

```
> espflash /dev/ttyUSBX target/xtensa-esp32-espidf/debug/blinky-esp32
```

where `/dev/ttyUSBX` is the serial port of the ESP32.
Make sure to do this on your system and not in the docker container.

### Via debugger (e.g. ESP-PROG)

Run:

```
> . $HOME/esp/esp-idf/export.sh
> openocd -f board/esp32-wrover-kit-3.3v.cfg -c "program_esp target/xtensa-esp32-espidf/debug/blinky-esp32.bin 0x10000 verify exit"
```

Make sure that you converted the ELF binary to an image!

## Debugging

### From a terminal

For most ESP32 boards an external debugger like [ESP-PROG](https://docs.espressif.com/projects/espressif-esp-iot-solution/en/latest/hw-reference/ESP-Prog_guide.html) is required.

Connect the ESP32 via the debugger to your computer and run

```
> . $HOME/esp/esp-idf/export.sh
> openocd -f board/esp32-wrover-kit-3.3v.cfg -c "program_esp target/xtensa-esp32-espidf/debug/blinky-esp32.bin 0x10000 verify reset"
```

Depending on your ESP32 you need to choose an appropriate board configuration. For most ESP devkits connected via ESP-PROG `board/esp32-wrover-kit-3.3v.cfg` is fine.

The `-c "..."` part can be omitted if the board is already flashed.

Keep the terminal open and from another one run

```
> . $HOME/esp/esp-idf/export.sh
> xtensa-esp32-elf-gdb -q target/xtensa-esp32-espidf/debug/blinky-esp32
(gdb) target extended-remote :3333
...
# Tell GDB where to find the sources because the binary was built in docker
(gdb) set substitute-path  /opt/project /path/to/the/project/embedded-rust-talk/blinky-platform-independent
(gdb) set substitute-path  /opt/cargo /path/to/your/home/dir/.cargo
(gdb) set substitute-path ... ... # and so on for all files you want to debug into...
(gdb) monitor reset halt
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

The project can be build by running the "Build ESP32 (docker)" task configured in `.vscode/tasks.json`.

A debugging session is started by running the "Debug ESP32 (openocd)" launch configuration configured in `.vscode/launch.json`.

## License notice

This example is based on the [Rust on ESP32 STD demo app](https://github.com/ivmarkov/rust-esp32-std-demo) by Ivan Markov licensed under the MIT license.

Original copyright and permission notice:

```
Copyright 2019-2020 Contributors to xtensa-lx6-rt

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