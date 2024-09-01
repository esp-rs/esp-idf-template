# Using WSL2 for esp-idf-template on Windows

This doc will help you set up and use WSL2 (Windows Subsystem for Linux 2) for development using the esp-idf-template on Windows. 
This method avoids [path length issues]([Error: Too long output directory · Issue #252 · esp-rs/esp-idf-sys (github.com)](https://github.com/esp-rs/esp-idf-sys/issues/252)) and provides a smoother development experience.

## System Requirements

- Windows 10 version 2004 and higher (Build 19041 and above) or Windows 11
- 64-bit processor with Second Level Address Translation (SLAT)
- 4GB system RAM or more (8GB recommended)
- BIOS-level hardware virtualization support must be enabled in the BIOS settings
- At least 5GB of free disk space for WSL2 and Ubuntu installation

## Basic prerequisite

- Administrative access to your Windows machine
- Reliable internet connection for downloading necessary components

## Setup Steps

1. Enable WSL2 on Windows
   - Open "Turn Windows features on or off"
   - Check "Virtual Machine Platform" and "Windows Subsystem for Linux"
   - Click OK and restart your computer when prompted

2. Install Ubuntu from Microsoft Store
   - Open Microsoft Store and search for "Ubuntu"
	   - As of September 2024, it's based on the Ubuntu 22.04 LTS distribution
   - Click "Install" to download and install Ubuntu

3. Launch Ubuntu and complete initial setup
   - Set up a username and password when prompted

4. Update and upgrade Ubuntu packages
   ```bash
   sudo apt update && sudo apt upgrade -y
   ```

5. Install required packages
   ```bash
   sudo apt install -y git curl gcc cmake python3 python3-pip pkg-config clang wget flex bison gperf python3-venv ninja-build ccache libffi-dev dfu-util libusb-1.0-0 libssl-dev libusb-1.0-0 libudev-dev
   ```

6. Install Rust and required tools
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   cargo install cargo-generate 
   cargo install ldproxy
   cargo install espup 
   cargo install espflash 
   cargo install cargo-espflash (optional)
   ```

7. Install ESP-IDF and toolchain
   ```bash
   espup install
   source ~/export-esp.sh
   ```

8. Configure USB access in WSL2
   - Install [usbipd-win](https://github.com/dorssel/usbipd-win/releases) on Windows
   - In your WSL2 terminal, run:
     ```bash
     sudo apt install linux-tools-generic hwdata
     sudo update-alternatives --install /usr/local/bin/usbip usbip /usr/lib/linux-tools/*-generic/usbip 20
     ```

## Using esp-idf-template with WSL2

1. Clone the project in WSL2:
   ```bash
   cargo generate esp-rs/esp-idf-template cargo
   ```

2. Navigate to your project directory:
   ```bash
   cd <your-project-name>
   ```

3. Build the project:
   ```bash
   cargo build
   ```

4. Connect ESP32 Device
   - [Link to windows doc](https://learn.microsoft.com/en-us/windows/wsl/connect-usb)
   - Plug in your ESP32 device
   - In PowerShell (as Administrator), run:
     ```powershell
     usbipd list
     usbipd bind --busid <busid>
     ```
   - Note the bus ID of your ESP32 device, then attach it to WSL2:
     ```powershell
     usbipd attach --wsl --busid <busid>
     ```

(cargo run OR > )
6. Flash the device:
   ```bash
   lsusb (check if device has attached)
   espflash flash target/xtensa-esp32-espidf/debug/<your-project-name>
   ```

7. Monitor the device:
   ```bash
   espflash monitor
   ```

## Troubleshooting

- If you encounter permission issues with the USB device, try running the flash and monitor commands with `sudo`.
- Ensure you've sourced the ESP-IDF environment in each new terminal session:
  ```bash
  source ~/export-esp.sh
  ```
- If you face any issues with USB device recognition, try restarting the WSL2 instance or your computer.
