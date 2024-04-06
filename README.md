# Rust on ESP-IDF "Hello, World" template
[![Cargo CI](https://github.com/esp-rs/esp-idf-template/actions/workflows/ci_cargo.yml/badge.svg)](https://github.com/esp-rs/esp-idf-template/actions/workflows/ci_cargo.yml)
[![CMake CI](https://github.com/esp-rs/esp-idf-template/actions/workflows/ci_cmake.yml/badge.svg)](https://github.com/esp-rs/esp-idf-template/actions/workflows/ci_cmake.yml)
[![Documentation](https://img.shields.io/badge/docs-esp--rs-brightgreen)](https://esp-rs.github.io/esp-idf-svc/esp_idf_svc/index.html)
[![Matrix](https://img.shields.io/matrix/esp-rs:matrix.org?label=join%20matrix&color=BEC5C9&logo=matrix)](https://matrix.to/#/#esp-rs:matrix.org)
[![Wokwi](https://img.shields.io/endpoint?url=https%3A%2F%2Fwokwi.com%2Fbadge%2Fclick-to-simulate.json)](https://wokwi.com/projects/332188235906155092)


A template for a "Hello, world!" Rust binary crate for the ESP-IDF framework.
Based on [cargo-generate](https://github.com/cargo-generate/cargo-generate).

This is the crate you get when running `cargo new`, but augmented with extra configuration so that it does build for the ESP32[XX] with ESP-IDF and (by default) with STD support.

Or if you rather
* ... want to mix Rust and C/C++ in a traditional ESP-IDF `idf.py` CMake project - [follow these instructions](README-cmake.md)
* ... want to mix Rust and C/C++ with PlatformIO - [follow these instructions](README-pio.md)

For more check out the links in the additional [information section](#additional-information)

## Generate the project

**Please make sure you have installed all [prerequisites](#prerequisites) first!**

```sh
cargo generate esp-rs/esp-idf-template cargo
```

The command will display a few prompts:
- `Project Name`: Name of the crate.
- `Which MCU to target?`: SoC model, e.g. `esp32`, `esp32s2`, `esp32c3` etc.
- `Configure advanced template options?`: If `false`, skips the rest of the prompts and uses their default value. If `true`, you will be prompted with:
  - `Enable STD support?`: When `true` (default), adds support for the [Rust Standard Library](https://doc.rust-lang.org/std/). Otherwise, a `no_std` [Rust Core Library](https://doc.rust-lang.org/core/index.html) crate would be created.
  - `ESP-IDF Version`: ESP-IDF branch/tag to use. Possible choices:
    - [`v4.4`](https://github.com/espressif/esp-idf/releases/tag/v4.4.5): Stable
    - [`v5.1`](https://github.com/espressif/esp-idf/releases/tag/v5.1): Stable
    - [`master`](https://github.com/espressif/esp-idf/tree/master): **Unstable**
   - `Configure project to support Wokwi simulation with Wokwi VS Code extension?`: Adds support for Wokwi simulation using [VS Code Wokwi extension](https://marketplace.visualstudio.com/items?itemName=wokwi.wokwi-vscode).
  - `Configure project to use Dev Containers (VS Code and GitHub Codespaces)?`: Adds support for:
      -  [VS Code Dev Containers](https://code.visualstudio.com/docs/remote/containers#_quick-start-open-an-existing-folder-in-a-container)
      -  [GitHub Codespaces](https://docs.github.com/en/codespaces/developing-in-codespaces/creating-a-codespace)
     Dev Containers also allow flashing from the container using [web flash](https://github.com/bjoernQ/esp-web-flash-server) and have the [VS Code Wokwi extension](https://marketplace.visualstudio.com/items?itemName=wokwi.wokwi-vscode) already installed.

## Build

```sh
cd <your-project-name>
cargo build
```

- Replace `<your-project-name>` with the name of the generated project

## Flash

In the root of the generated project:

```sh
espflash flash target/<mcu-target>/debug/<your-project-name>
```

| MCU      | Target                    |
| -------- | ------------------------- |
| ESP32    | `xtensa-esp32-espidf`     |
| ESP32-S2 | `xtensa-esp32s2-espidf`   |
| ESP32-S3 | `xtensa-esp32s3-espidf`   |
| ESP32-C2 | `riscv32imc-esp-espidf`   |
| ESP32-C3 | `riscv32imc-esp-espidf`   |
| ESP32-C6 | `riscv32imac-esp-espidf`  |
| ESP32-H2 | `riscv32imac-esp-espidf`  |
| ESP32-P4 | `riscv32imafc-esp-espidf` |

- `espflash` will print a list of the recognized USB ports for you to select
the desired port, if it detectes multiple boards.
- Replace `<your-project-name>` with the name of the generated project
- You can include the `--monitor` argument to the `espflash` command to open a serial monitor after flashing the device.
- For more details on [`espflash` usage see the README](https://github.com/esp-rs/espflash/tree/main/espflash#usage)

## Monitor
```sh
espflash monitor /dev/ttyUSB0
```

- Replace `dev/ttyUSB0` above with the USB port where you've connected the board. If you do not
specify any USB port, `cargo-espflash`/`espflash` will print a list of the recognized USB ports for you to select
the desired port.

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

## Additional information

For more information, check out:
* The [Rust on ESP Book](https://esp-rs.github.io/book/)
* The [ESP STD Embedded Training](https://github.com/esp-rs/std-training)
* The [esp-idf-hal](https://github.com/esp-rs/esp-idf-hal) project
* The [embedded-hal](https://github.com/rust-embedded/embedded-hal) project
* The [esp-idf-svc](https://github.com/esp-rs/esp-idf-svc) project
* The [embedded-svc](https://github.com/esp-rs/embedded-svc) project
* The [esp-idf-sys](https://github.com/esp-rs/esp-idf-sys) project
* The [Rust for Xtensa toolchain](https://github.com/esp-rs/rust-build)
* The [Rust-with-STD demo](https://github.com/ivmarkov/rust-esp32-std-demo) project

## Prerequisites

Linux/Mac users: Make sure you have the dependencies installed,that are mentiond in the [esp-idf install guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/linux-macos-setup.html#step-1-install-prerequisites). You **dont** need to manually install esp-idf, just its dependencies.

For detailed instructions see [Setting Up a Development Environment](https://esp-rs.github.io/book/installation/index.html) chapter of The Rust on ESP Book.

### Install Rust (with `rustup`)

If you don't have `rustup` installed yet, follow the instructions on the [rustup.rs site](https://rustup.rs)

### Install Cargo Sub-Commands

```sh
cargo install cargo-generate
cargo install ldproxy
cargo install espup
cargo install espflash
cargo install cargo-espflash # Optional
```

> [!NOTE]
> If you are running Linux then `libudev` must also be installed for `espflash` and `cargo-espflash`; this is available via most popular package managers. If you are running Windows you can ignore this step.

> ```
> # Debian/Ubuntu/etc.
> apt-get install libudev-dev
> # Fedora
> dnf install systemd-devel
> ```

> Also, the `espflash` and `cargo-espflash` commands shown below, assume that version `2.0` or
> greater.

### Install Rust & Clang toolchains for Espressif SoCs (with `espup`)

```sh
espup install
# Unix
. $HOME/export-esp.sh
```
> [!WARNING]
> Make sure you source the generated export file, as shown above, in every terminal before building any application as it contains the required environment variables.

See the [Installation chapter of The Rust on ESP Book](https://esp-rs.github.io/book/installation/index.html) for more details.

### Alternative (for RISC-V Espressif SOCs **only**): install & use upstream nightly Rust and upstream stable Clang

While you **can** target the RISC-V Espressif SOCs (`esp32-cXX` and `esp32-hXX`) with the `espup` installer just fine, SOCs with this architecture are also [supported by the nightly Rust compiler](https://esp-rs.github.io/book/installation/riscv.html) and by recent, stock Clang compilers (as in Clang 11+):

* Install a recent Clang. See [Clang Getting Started page](https://clang.llvm.org/get_started.html) as it contains useful guidelines on installation. Recent Linux distros come with suitable Clang already.
* Install the `nightly` Rust toolchain with the `rust-src` component included:
   ```sh
   rustup toolchain install nightly --component rust-src
   ```
* Run any Cargo command with the `nightly` [toolchain override](https://rust-lang.github.io/rustup/overrides.html#overrides), i.e. `cargo +nightly ...`.

### Install Python3

You need a Python 3.7 or later installed on your machine.
* Linux, Mac OS X: if not preinstalled already, just install it with your package manager, i.e. for Debian systems: `sudo apt install python3`
* Windows: install it e.g. [from the official Python site](https://www.python.org/downloads/).

You'll also need the Python PIP and Python VENV modules. On Debian systems, you can install with:
* `sudo apt install python3-pip python3-venv`

