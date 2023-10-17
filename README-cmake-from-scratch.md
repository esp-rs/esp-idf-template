# Integrating a Rust Component into an ESP-IDF Project

ESP-IDF, the official development framework for the ESP32 Series SoCs, supports integration of components written in C/C++ and Rust which is gaining traction for embedded development due to its safety features. This article outlines the steps to add a Rust component to your ESP-IDF project.

## Prerequisites

- [Installed ESP-IDF](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/index.html#installation)
- [Installed Rust and Cargo](https://www.rust-lang.org/tools/install)
- [Installed Xtensa LLVM toolchain for Rust](https://esp-rs.github.io/book/installation/index.html)
- Basic knowledge of ESP-IDF, CMake, and Rust

## Structure

Here's how your project directory might look after the following the guide:

```
esp-idf-project/
|-- CMakeLists.txt
|-- main/
|   |-- CMakeLists.txt
|   |-- esp-idf-project.c
|-- sdkconfig
|-- components/
|   |-- esp-rust-component/
|       |-- CMakeLists.txt
|       |-- include/
|           |-- esp-rust-component.h
|       |-- esp-rust-component.c
|       |-- rust-crate/
|           |-- Cargo.toml
|           |-- rust-toolchain.toml
|           |-- src/
|               |-- lib.rs
```

### Architecture

Key elements:
- `esp-idf-project` contains main C code like any other ESP-IDF application.
- The ESP-IDF componet with name `esp-rust-component` is stored in subdirectory with components.
  - The `esp-rust-component` component contains C adapter layer, which helps interfacing with Rust library.
- The Rust code is stored in `components/esp-rust-component/rust-crate` subdirectory.

The component can be uploaded later to [Component Manager](https://components.espressif.com/).

## Step-by-Step Guide

### Set-up the environment

Before starting the project, make sure that the [Prerequisites](#prerequisites) are met, and that you have sourced the required export files.

### Create ESP-IDF project

Use ESP-IDF tooling to create new project with name `esp-idf-project`.

```
idf.py create-project esp-idf-project
cd esp-idf-project
```

### Create the ESP-IDF Component

Create a new directory in your `components/` folder. You can name it `esp-rust-component`.

```
mkdir components
cd components
idf.py create-component esp-rust-component
```

If you've decided to create *new* component manually make sure to run, so that CMake will pick newly created component:

```
idf.py reconfigure
```

### Set up the CMakeLists.txt File

In your `esp-rust-component` directory, edit the [`CMakeLists.txt`](./components/esp-rust-component/CMakeLists.txt) file with the following content:

```cmake
idf_component_register(
    SRCS "esp-rust-component.c"
    INCLUDE_DIRS "include"
)

# Define the Rust target for the Xtensa and RISC-V architecture
if (CONFIG_IDF_TARGET_ARCH_XTENSA)
    set(RUST_CARGO_TOOLCHAIN "+esp")
    set(RUST_CARGO_TARGET "xtensa-${IDF_TARGET}-none-elf")
elseif (CONFIG_IDF_TARGET_ARCH_RISCV)
    set(RUST_CARGO_TOOLCHAIN "+nightly")
    set(RUST_CARGO_TARGET "riscv32imac-unknown-none-elf")
else()
    message(FATAL_ERROR "Architecture currently not supported")
endif()

# Set the flags for cargo build
set(CARGO_BUILD_FLAGS "-Zbuild-std=core")

# Set directories and target
set(RUST_PROJECT_DIR "${CMAKE_CURRENT_LIST_DIR}/rust-crate")
set(RUST_BUILD_DIR "${CMAKE_CURRENT_BINARY_DIR}")
set(RUST_TARGET_DIR "${RUST_BUILD_DIR}/target")
set(RUST_STATIC_LIBRARY "${RUST_TARGET_DIR}/${RUST_CARGO_TARGET}/release/librust_crate.a")

# ExternalProject_Add for building the Rust project
ExternalProject_Add(
    rust_crate_target
    PREFIX "${RUST_PROJECT_DIR}"
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND ${CMAKE_COMMAND} -E env
        CARGO_BUILD_TARGET=${RUST_CARGO_TARGET}
        CARGO_BUILD_TARGET_DIR=${RUST_TARGET_DIR}
        cargo ${RUST_CARGO_TOOLCHAIN} build --release ${CARGO_BUILD_FLAGS} -Zbuild-std-features=compiler-builtins-weak-intrinsics
    BUILD_ALWAYS TRUE
    INSTALL_COMMAND ""
    WORKING_DIRECTORY ${RUST_PROJECT_DIR}
    TMP_DIR "${RUST_BUILD_DIR}/tmp"
    STAMP_DIR "${RUST_BUILD_DIR}/stamp"
    DOWNLOAD_DIR "${RUST_BUILD_DIR}"
    SOURCE_DIR "${RUST_PROJECT_DIR}"
    BINARY_DIR "${RUST_PROJECT_DIR}"
    INSTALL_DIR "${RUST_BUILD_DIR}"
    BUILD_BYPRODUCTS "${RUST_STATIC_LIBRARY}"
)

# Add prebuilt Rust library
add_prebuilt_library(rust_crate_lib "${RUST_STATIC_LIBRARY}" REQUIRES "")

# Add dependencies and link Rust library
add_dependencies(${COMPONENT_LIB} rust_crate_target)
target_link_libraries(${COMPONENT_LIB} PUBLIC rust_crate_lib)
```

### Create a Rust Project Inside the Component

Create a new Rust crate, which will be a library, inside `esp-rust-component` called `rust-crate`:

```bash
cargo init --lib rust-crate
```

Update the [`Cargo.toml`](./components/esp-rust-component/rust-crate/Cargo.toml) to match the settings for your target board. Also set the crate type to `staticlib`:

```toml
[package]
name = "rust-crate"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["staticlib"]

[dependencies]

[features]
default = [ ]
```

### Rust to C Interoperability

Add a Rust function with C linkage in your [`lib.rs`](./components/esp-rust-component/rust-crate/src/lib.rs) that will be callable from C code. An example might be:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

use core::ffi::c_void;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}

static HELLO_ESP32: &'static [u8] = b"Hello ESP-RS. https://github.com/esp-rs\0";

#[no_mangle]
pub extern "C" fn hello() -> *const c_void {
    HELLO_ESP32.as_ptr() as *const c_void
}
```

### Create a C Wrapper

Create file [`esp-rust-component/esp-rust-component.c`](./components/esp-rust-component/esp-rust-component.c) to include the Rust functions.

```c
#include "rust_component.h"
```

### Update the Header File

Include the C header file in your [`esp-rust-component/include/esp-rust-component.h`](./components/esp-rust-component/include/esp-rust-component.h):

```c
extern const void* hello();
```

### Call Rust code from C

Update main ESP-IDF project file [`main/esp-idf-project.c`](./main/esp-idf-project.c):

```c
#include "stdio.h"
#include "esp-rust-component.h"

void app_main() {
    const char* message = hello();
    printf("%s\n", message);
}
```

### Select target

Set target for main ESP-IDF application:

```bash
idf.py set-target <target>
# idf.py set-target esp32
# idf.py set-target esp32-c3
# idf.py set-target esp32-s3
```

Optional step when developers need to build Rust components manually:
Define which toolchain should be used for the Rust component in file `esp-rust-component/rust-crate/rust-toolchain.toml`

```toml
[toolchain]
# Use "esp" for ESP32, ESP32-S2, and ESP32-S3
channel = "esp"
# Use "nightly" for ESP32-C*, ESP32-H* targets
# channel = "nightly"

```


### Build the Project
From the base folder of the project (`esp-idf-project`), run the build process as you usually would for an ESP-IDF project:

```bash
idf.py build flash monitor
```

This command will build, flash the resulting binary to your board and open a serial monitor.

## Troubleshooting

- If you encounter linker errors, you may need to update your Rust flags. For example, you might need to add the `-Zbuild-std-features=compiler-builtins-weak-intrinsics` flag to `CARGO_BUILD_FLAGS` in your `CMakeLists.txt`.

## Simulation

### Simulation with Wokwi in VS Code

[Wokwi for Visual Studio Code](https://docs.wokwi.com/vscode/getting-started) provides a simulation solution for embedded and IoT system engineers. The extension integrates with your existing development environment, allowing you to simulate your projects directly from your code editor.

- [Install VS Code](https://code.visualstudio.com/download)
- [Install Wokwi plugin](https://docs.wokwi.com/vscode/getting-started#installation)
- Activate Wokwi plugin - command palette, search for `Wokwi: Start Simulator`, select and activate the plugin using web browser

### Add files for Wokwi simulator

Create [`wokwi.toml`](./wokwi.toml) in the root of the project. The file contains references to BIN and ELF previously built by `idf.py`.

```toml
[wokwi]
version = 1
elf = "build/esp-idf-project.elf"
firmware = "build/esp-idf-project.bin"
```

Create [`diagram.json`](./diagram.json). The file contains board selected for the simulation.

```json
{
  "version": 1,
  "author": "Espressif Systems",
  "editor": "wokwi",
  "parts": [ { "type": "wokwi-esp32-devkit-v1", "id": "esp", "top": 0, "left": 0, "attrs": {} } ],
  "connections": [ [ "esp:TX0", "$serialMonitor:RX", "", [] ], [ "esp:RX0", "$serialMonitor:TX", "", [] ] ],
  "dependencies": {}
}
```

Open VS Code, open command palette (CMD/Ctrl+Shift+P), search for `Wokwi: Start Simulator`, select the option to start simulation.

Use Pause button to display state of pins.

The plugin auto-reload application if the binary was updated.

## Using this repository as template

If you prefer starting the project from a template, you can use different methods:
- `git`: Simply clone the repository and avoid generating and populating the different files
- [cargo-generate](https://github.com/cargo-generate/cargo-generate) to get the project ready to build and flash!
- [GitHub templates](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template)

### `git`

1. Make sure that the [Prerequisites](#prerequisites) are met and that you have sourced the required export files.
2. Clone the repository: `git clone https://github.com/georgik/esp32-idf-no-std-rust-component`
3. [Set the target](https://github.com/georgik/esp32-idf-no-std-rust-component?tab=readme-ov-file#select-target): `idf.py set-target <target>`
4. [Build and flash the project](https://github.com/georgik/esp32-idf-no-std-rust-component?tab=readme-ov-file#build-the-project): `idf.py build flash monitor`


### `cargo-generate`

1. Make sure that the [Prerequisites](#prerequisites) are met and that you have sourced the required export files.
2. Install `cargo-generate`: `cargo install cargo-generate`
3. Generate the template `cargo generate georgik/esp32-idf-no-std-rust-component`
4. [Set the target](https://github.com/georgik/esp32-idf-no-std-rust-component?tab=readme-ov-file#select-target): `idf.py set-target <target>`
5. [Build and flash the project](https://github.com/georgik/esp32-idf-no-std-rust-component?tab=readme-ov-file#build-the-project): `idf.py build flash monitor`

### GitHub templates

1. Make sure that the [Prerequisites](#prerequisites) are met and that you have sourced the required export files.
2. [Create your own repository](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template#creating-a-repository-from-a-template) from [`georgik/esp32-idf-no-std-rust-component`](https://github.com/georgik/esp32-idf-no-std-rust-component)
   1. Above the file list, click Use this template.
   2. Select Create a new repository.
   3. Fill the required information
   4. Create the repository
3. Clone the repository that you just created: `git clone https://github.com/<owner>/<repository>`
4. [Set the target](https://github.com/georgik/esp32-idf-no-std-rust-component?tab=readme-ov-file#select-target): `idf.py set-target <target>`
5. [Build and flash the project](https://github.com/georgik/esp32-idf-no-std-rust-component?tab=readme-ov-file#build-the-project): `idf.py build flash monitor`

## Adding GitHub Action tests

If we want to add some CI to our project, we can leverage [Wokwi CI](https://docs.wokwi.com/wokwi-ci/getting-started), the following
YAML file will build and check that the generated project runs properly:

```yaml
name: CI
on:
  push:
  pull_request:
  workflow_dispatch:

env:
  CARGO_TERM_COLOR: always
  GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  ESP_TARGET: esp32
  ESP_IDF_VERSION: v5.1


jobs:
  build-check:
    name: Checks
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup | Rust (RISC-V)
        if: env.ESP_TARGET != 'esp32' && env.ESP_TARGET != 'esp32s2' && env.ESP_TARGET != 'esp32s3'
        uses: dtolnay/rust-toolchain@v1
        with:
          target: riscv32imc-unknown-none-elf
          toolchain: nightly
          components: rust-src
      - name: Setup | Rust (Xtensa)
        if: env.ESP_TARGET == 'esp32' && env.ESP_TARGET == 'esp32s2' && env.ESP_TARGET == 'esp32s3'
        uses: esp-rs/xtensa-toolchain@v1.5
        with:
          default: true
          buildtargets: {{ env.ESP_TARGET }}
          ldproxy: false
      - uses: Swatinem/rust-cache@v2
      - name: Setup | ESP-IDF
        shell: bash
        run: |
          git clone -b {{ env.ESP_IDF_VERSION }} --shallow-submodules --single-branch --recursive https://github.com/espressif/esp-idf.git /home/runner/work/esp-idf
          /home/runner/work/esp-idf/install.sh  {{ env.ESP_TARGET }}
      - name: Build project
        shell: bash
        run: |
          . /home/runner/work/esp-idf/export.sh
          idf.py set-target  {{ env.ESP_TARGET }}
          idf.py build
      - name: Wokwi CI check
        uses: wokwi/wokwi-ci-action@v1
        with:
          token: ${{ secrets.WOKWI_CLI_TOKEN }}
          timeout: 10000
          expect_text: 'Hello ESP-RS. https://github.com/esp-rs'
          fail_text: 'Error'
```

I'ts important to note that we need to set the `WOKWI_CLI_TOKEN` secret:
1. [Create a Wokwi CI token](https://wokwi.com/dashboard/ci)
2. [Add it as a secret in your GitHub repository](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions)

Also, the CI file needs to be modified if used for other targets.
