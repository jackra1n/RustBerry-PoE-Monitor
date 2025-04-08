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
    *   `brightness` (Default: `2`): Sets the OLED brightness level (0 = dimmest, 4 = brightest).
    *   `screen_timeout` (Default: `300`): Time in seconds before the screen dims to the lowest brightness (set to 0 to disable timeout).
    *   `enable_periodic_off` (Default: `false`): Set to `true` to enable the periodic on/off cycle.
    *   `periodic_on_duration` (Default: `10`): If periodic off is enabled, duration (seconds) the display stays ON.
    *   `periodic_off_duration` (Default: `20`): If periodic off is enabled, duration (seconds) the display stays OFF.
    *   `refresh_interval_ms` (Default: `1000`): How often (in milliseconds) the display and system stats are updated. Lower values are faster but use more CPU.
*   **`[fan]`**
    *   `temp_on` (Default: `60.0`): CPU temperature (Celsius) at which the fan turns on.
    *   `temp_off` (Default: `50.0`): CPU temperature (Celsius) at which the fan turns off.

**Example `config.toml`:**

```toml
# RustBerry-PoE-Monitor Configuration

[display]
# Brightness level: 0 (dimmest) to 4 (brightest)
# Default: 2
brightness = 2

# Screen timeout settings (in seconds)
# Time before screen dims to the lowest brightness (set to 0 to disable timeout).
# Default: 300
screen_timeout = 300

# Periodic Display Off Feature
# Default: false
enable_periodic_off = false
# Duration (in seconds) the display stays ON before turning OFF periodically.
# Default: 10
periodic_on_duration = 10
# Duration (in seconds) the display stays OFF periodically.
# Default: 20
periodic_off_duration = 20

# Refresh interval for the display update loop (in milliseconds).
# Lower values update faster but use slightly more CPU.
# Default: 1000 (1 second)
refresh_interval_ms = 1000

[fan]
# Temperature thresholds for fan control (Celsius)
# Default: 60.0
temp_on = 60.0   # Temperature at which the fan turns on
# Default: 50.0
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
