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

To change the fan on/off temperature, you can add `temp-on` and `temp-off` arguments to the application.
If you installed the application using the install script, you can edit the systemd service file to change the arguments.

```bash
sudo nano /etc/systemd/system/rustberry-poe-monitor.service 
```

Change the `ExecStart` line to the following:
```bash
ExecStart=/usr/local/bin/rustberry-poe-monitor --temp-on 60 --temp-off 50
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
