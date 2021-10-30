# Rust on ESP32 STD "Hello, World" template

A "Hello, world!" template of a STD binary crate for the ESP32[XX] and ESP-IDF.

This is the crate you get when running `cargo new`, but augmented with extra configuration so that it does build for the ESP32[XX] with ESP-IDF and STD support.

![CI](https://github.com/ivmarkov/rust-esp32-std-mini/actions/workflows/ci.yml/badge.svg)

  ## Prerequisites

### Rustup

If you don't have `rustup` installed yet, follow the instructions on the [rustup.rs site](https://rustup.rs)

### Rust & Clang - for Xtensa MCUs (ESP32, ESP32-S2 and ESP32-S3)

- Install the [Rust Espressif compiler toolchain and the Espressif LLVM Clang toolchain](https://github.com/esp-rs/rust-build)
- This is necessary, because support for the Xtensa architecture (ESP32 / ESP32-S2 / ESP32-S3) is not upstreamed in LLVM yet
- Make sure that you DON'T have a system Clang installed as well, because even if you have the Espressif one first on your `$PATH`, Bindgen will still pick the system one
  - A workaround that does not require uninstalling the system Clang is to do `export LIBCLANG_PATH=<path to the Espressif Clang lib directory>` prior to continuing the build process

### Rust & Clang - for RiscV32 MCUs (ESP32-C3)

- You **can** target the ESP32-C3 with the Espressif toolchains just fine, but this MCU is also supported by the stock compilers
- Just use the stock nightly Rust compiler, and a recent, stock Clang (as in Clang 11+)

To install the nightly Rust compiler toolchain:
```sh
rustup install nightly
rustup component add rust-src --toolchain nightly
```

Installing a recent Clang compiler is OS-specific. The [Clang Getting Started page](https://clang.llvm.org/get_started.html) contains useful guidelines.

### Cargo Sub-Commands

```sh
cargo install cargo-generate
cargo install ldproxy
cargo install espflash
cargo install espmonitor
```

## Generate the project

```sh
cargo generate --vcs none --git https://github.com/ivmarkov/rust-esp32-std-template
```

## Build

To build using the default PlatformIO builder just use:
```sh
cargo build
```

To build using the ESP-IDF native builder, use:
```sh
cargo build --features native
```

## Flash

In the root of the generated project:

```sh
espflash /dev/ttyUSB0 target/[xtensa-esp32-espidf|xtensa-esp32s2-espidf|xtensa-esp32s3-espidf|riscv32imc-esp-espidf]/debug/<your-project-name>
```

- Replace `dev/ttyUSB0` above with the USB port where you've connected the board
- Replace `<your-project-name>` with the name of the generated project

## Monitor

```sh
espmonitor /dev/ttyUSB0
```

- Replace `dev/ttyUSB0` above with the USB port where you've connected the board


The monitor should output more or less the following:
```
Opening /dev/tty.usbserial-0001 with speed 115200
Resetting device... done
ets Jun  8 2016 00:22:57

rst:0x1 (POWERON_RESET),boot:0x13 (SPI_FAST_FLASH_BOOT)
configsip: 0, SPIWP:0xee
clk_drv:0x00,q_drv:0x00,d_drv:0x00,cs0_drv:0x00,hd_drv:0x00,wp_drv:0x00
mode:DIO, clock div:2
load:0x3fff0048,len:12
ho 0 tail 12 room 4
load:0x3fff0054,len:4800
load:0x40078000,len:17448
load:0x4007c428,len:4840
entry 0x4007c6a0
I (178) cpu_start: Pro cpu up.
I (178) cpu_start: Starting app cpu, entry point is 0x4008115c
I (0) cpu_start: App cpu up.
I (193) cpu_start: Pro cpu start user code
I (193) cpu_start: cpu freq: 160000000
I (193) cpu_start: Application information:
I (197) cpu_start: Project name:     esp-idf
I (202) cpu_start: App version:      f08dcd7
I (207) cpu_start: Compile time:     Oct 23 2021 14:48:03
I (213) cpu_start: ELF file SHA256:  0000000000000000...
I (219) cpu_start: ESP-IDF:          4.3.0
I (224) heap_init: Initializing. RAM available for dynamic allocation:
I (231) heap_init: At 3FFAE6E0 len 00001920 (6 KiB): DRAM
I (237) heap_init: At 3FFB3498 len 0002CB68 (178 KiB): DRAM
I (243) heap_init: At 3FFE0440 len 00003AE0 (14 KiB): D/IRAM
I (250) heap_init: At 3FFE4350 len 0001BCB0 (111 KiB): D/IRAM
I (256) heap_init: At 4008C538 len 00013AC8 (78 KiB): IRAM
I (263) spi_flash: detected chip: generic
I (267) spi_flash: flash io: dio
I (272) cpu_start: Starting scheduler on PRO CPU.
I (0) cpu_start: Starting scheduler on APP CPU.
Hello, world!
```
