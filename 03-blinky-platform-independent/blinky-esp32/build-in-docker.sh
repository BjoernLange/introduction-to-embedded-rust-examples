#!/bin/bash

set -e

rustup default esp
cd /opt/project/blinky-esp32
cargo build
#esptool.py --chip ESP32 elf2image target/xtensa-esp32-espidf/debug/blinky-esp32
