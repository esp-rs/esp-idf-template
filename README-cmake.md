# Rust on ESP-IDF "Hello, World" CMake template

A "Hello, world!" template of a mixed Rust/C ESP-IDF project driven by `idf.py` and CMake.

## Generate the project

**Please make sure you have installed all [prerequisites](#prerequisites) first!**

```sh
cargo generate --vcs none --git https://github.com/esp-rs/esp-idf-template cmake
```

The command will display a few prompts:
  - `Project Name`: Name of the crate.
  - `Rust toolchain`: Selects the `channel` in the [toolchain file](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file) to use. Select `nightly` for ESP32-C3 and `esp` for the other targets.
  - `STD support`: When `true`, adds support for [Rust Standard Library](https://doc.rust-lang.org/std/). Otherwise, we will use [Rust Core Library](https://doc.rust-lang.org/core/index.html).

## Enable ESP IDF components that you would like to use from Rust

Contrary to the [cargo-first](https://github.com/esp-rs/esp-idf-template/blob/master/README.md) build, the CMake build - by default - runs with very few ESP IDF components enabled for Rust. If you want to enable extra components so that they are usable via their Rust wrappers, you need to list them [here](https://github.com/esp-rs/esp-idf-template/blob/master/cmake/components/rust-%7B%7Bproject-name%7D%7D/CMakeLists.txt#L3) **in your newly-generated project**.

## Build

```sh
cd <your-project-name>
idf.py set-target [esp32|esp32s2|esp32s3|esp32c3]
idf.py build
```

- Replace `<your-project-name>` with the name of the generated project

## Flash

```sh
idf.py -p /dev/ttyUSB0 flash
```

- Replace `dev/ttyUSB0` above with the USB port where you've connected the board

## Monitor

```sh
idf.py -p /dev/ttyUSB0 monitor
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

## Prerequisites

### Install Rust (with `rustup`)

If you don't have `rustup` installed yet, follow the instructions on the [rustup.rs site](https://rustup.rs)

### Install Cargo Sub-Commands

```sh
cargo install cargo-generate
cargo install espup
```

### Install Rust & Clang toolchains for Espressif SoCs (with `espup`)

```sh
espup install
# Unix
. $HOME/export-esp.sh
# Windows
%USERPROFILE%\export-esp.ps1
```
> **Warning**
>
> Make sure you source the generated export file, as shown above, in every terminal before building any application as it contains the required environment variables.

See the [Installation chapter of The Rust on ESP Book](https://esp-rs.github.io/book/installation/installation.html) for more details.

### Alternative (for RISC-V Espressif SOCs **only**): install & use upstream Rust & Clang

While you **can** target the RISC-V Espressif SOCs (`esp32-cXX` and `esp32-hXX`) with the `espup` installer just fine, SOCs with this architecture are also [supported by the nightly Rust compiler](https://esp-rs.github.io/book/installation/installation.html#risc-v) and by recent, stock Clang compilers (as in Clang 11+):

1. Install a recent Clang. See [Clang Getting Started page](https://clang.llvm.org/get_started.html) as it contains useful guidelines on instalaltion. Recent Linux distros come with suitable Clang already.
2. Install the `nightly` Rust toolchain with the `rust-src` component included:
   ```sh
   rustup toolchain install nightly --component rust-src
   ```
   
### Install Python3

You need a Python 3.7 or later installed on your machine. Install it from the package distro of your OS, or download and install it [from the official Python site](https://www.python.org/downloads/).

### Install ESP-IDF SDK & Tooling

When using `idf.py` and CMake driven ESP-IDF projects, you need to [install the ESP-IDF SDK and its tooling manually](https://docs.espressif.com/projects/esp-idf/en/v4.3.1/esp32/get-started/index.html).

Simple installation for Linux & MacOS:
```sh
git clone https://github.com/espressif/esp-idf
git -C esp-idf checkout release/v4.4
esp-idf/install.sh
. esp-idf/export.sh
```

Simple installation for Windows:
```sh
git clone https://github.com/espressif/esp-idf
git -C esp-idf checkout release/v4.4
esp-idf\install
esp-idf\export
```
