# 🦀🍇 RustBerry-PoE-Monitor

[![Build](https://github.com/jackra1n/RustBerry-PoE-Monitor/actions/workflows/build.yaml/badge.svg)](https://github.com/jackra1n/RustBerry-PoE-Monitor/actions/workflows/build.yaml)
![Rust](https://img.shields.io/badge/Rust-%23000000.svg?logo=rust&logoColor=white)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
![Raspi](https://img.shields.io/badge/Raspberry%20Pi-A22846?logo=Raspberry%20Pi&logoColor=white)

RustBerry-PoE-Monitor is a Rust-based monitoring and control tool for the Raspberry Pi, specifically designed for use with the [Waveshare PoE HAT (B)](https://www.waveshare.com/wiki/PoE_HAT_(B)). 

![Example image](/docs/IMG_3890.webp)

This tool provides real-time monitoring of your Raspberry Pi's system statistics, including IP address, memory usage, CPU load, and more, displayed on the PoE HAT's OLED display. Additionally, it offers fan control to maintain optimal operating temperatures.

## 📖 Table of Contents

- [🦀🍇 RustBerry-PoE-Monitor](#-rustberry-poe-monitor)
  - [📖 Table of Contents](#-table-of-contents)
  - [🌟 Features](#-features)
  - [📦 Installation](#-installation)
    - [Easy Installation](#easy-installation)
    - [Manual Installation](#manual-installation)
  - [📝 Configuration](#-configuration)
  - [🛠️ Building](#️-building)
    - [Prerequisites](#prerequisites)
    - [Building for Raspberry Pi](#building-for-raspberry-pi)
  - [🏃‍♂️ Running](#️-running)


## 🌟 Features

- **Minimal** resource usage
- Developed in Rust for memory safety 🦀
- Display real-time system statistics (IP address, memory, CPU usage, etc.) on the PoE HAT's OLED screen 📊
- Automatic fan control based on the CPU temperature 🌡️

## 📦 Installation

### Easy Installation

Run the following command to install:
```bash
curl -sSL https://rustberry.jackra1n.com/install | sudo bash
```

And that's it!

### Manual Installation

Check out the [wiki page](https://github.com/jackra1n/RustBerry-PoE-Monitor/wiki/Manual-Installation) to learn how to install manually.

## 📝 Configuration

Configuration is handled via a TOML file located at:

```
$HOME/.config/rustberry-poe-monitor/config.toml
```

If this file does not exist when you first run the application, it will be automatically created with default settings. You can then edit this file to customize the behavior.

**Available Options:**

*   **`[display]`**
    *   `brightness`: Sets the OLED brightness level (0 = dimmest, 4 = brightest).
    *   `screen_timeout`: Time in seconds before the screen dims to the lowest brightness (set to 0 to disable timeout).
*   **`[fan]`**
    *   `temp_on`: CPU temperature (Celsius) at which the fan turns on.
    *   `temp_off`: CPU temperature (Celsius) at which the fan turns off.

**Example `config.toml`:**

```toml
# RustBerry-PoE-Monitor Configuration

[display]
# Brightness level: 0 (dimmest) to 4 (brightest)
brightness = 2

# Screen timeout settings (in seconds)
# Time before screen dims to the lowest brightness (set to 0 to disable timeout).
screen_timeout = 300

[fan]
# Temperature thresholds for fan control (Celsius)
temp_on = 60.0   # Temperature at which the fan turns on
temp_off = 50.0  # Temperature at which the fan turns off
```

## 🛠️ Building

### Prerequisites
For building for Rasberry Pi I'm using [cross](https://github.com/cross-rs/cross)

### Building for Raspberry Pi
```bash
 cross build --target=aarch64-unknown-linux-gnu --release
```

## 🏃‍♂️ Running

Just run the binary file
```bash
./rustberry-poe-monitor
```


## Links

- [Waveshare PoE HAT (B)](https://www.waveshare.com/wiki/PoE_HAT_(B))
- [raspi-poe-mon](https://github.com/klamann/raspi-poe-mon) - python implementation and original idea for the display layout
- [PCSenior](https://www.1001fonts.com/pc-senior-font.html) - Font used for the display
- [cross](https://github.com/cross-rs/cross) - Rust tool for building cross-platform binaries
