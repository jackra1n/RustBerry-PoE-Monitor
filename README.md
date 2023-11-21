# 🦀🍇 RustBerry-PoE-Monitor

[![Build](https://github.com/jackra1n/RustBerry-PoE-Monitor/actions/workflows/build.yaml/badge.svg)](https://github.com/jackra1n/RustBerry-PoE-Monitor/actions/workflows/build.yaml)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?logo=rust&logoColor=white)
![Raspi](https://img.shields.io/badge/Raspberry%20Pi-A22846?logo=Raspberry%20Pi&logoColor=white)

RustBerry-PoE-Monitor is a Rust-based monitoring and control tool for the Raspberry Pi, specifically designed for use with the [Waveshare PoE HAT (B)](https://www.waveshare.com/wiki/PoE_HAT_(B)). 

![Example image](/docs/IMG_3890.webp)

This tool provides real-time monitoring of your Raspberry Pi's system statistics, including IP address, memory usage, CPU load, and more, displayed on the PoE HAT's OLED display. Additionally, it offers fan control to maintain optimal operating temperatures.

## 📖 Table of Contents

1. [Features](#🌟-features)
2. [Installation](#📦-installation)
    1. [Download](#📥-download)
        1. [Pre-built binaries](#pre-built-binaries)
        2. [Cargo](#cargo)
    2. [Configuration](#📝-configuration)
3. [Building](#🛠️-building)


## 🌟 Features

- **Minimal** resource usage
- Developed in Rust for memory safety 🦀
- Display real-time system statistics (IP address, memory, CPU usage, etc.) on the PoE HAT's OLED screen 📊
- Automatic fan control based on the CPU temperature 🌡️

## 📦 Installation

### 📥 Download
First, you will need the binary file. You can either download the pre-built binaries or download it using cargo.

#### Pre-built binaries
Pre-built binaries are available for download on the [releases page](https://github.com/jackra1n/RustBerry-PoE-Monitor/releases)

To be able to run the binary anywhere on your system, you can move it to `/usr/local/bin`:
```bash
sudo mv rustberry-poe-monitor /usr/local/bin
```

#### Cargo

Install with `cargo`:
```bash
cargo install rustberry-poe-monitor
```

### 📝 Configuration

You should be able to run the binary file now:
```bash
rustberry-poe-monitor
```

There are 2 CLI arguments available:
- `--temp-on` - The temperature at which the fan should turn on (default: 60)
- `--temp-off` - The temperature at which the fan should turn off (default: 50)

Example:
```bash
rustberry-poe-monitor --temp-on 65 --temp-off 55
```

To run the program on startup, you can create a systemd service:
```bash
sudo nano /etc/systemd/system/rustberry-poe-monitor.service
```

Paste the following into the file:
```bash
[Unit]
Description=RustBerry PoE Monitor
After=network.target

[Service]
ExecStart=/home/yourUser/.cargo/bin/rustberry-poe-monitor
User=yourUser
Restart=always
RestartSec=30

[Install]
WantedBy=multi-user.target
```

Then enable the service:
```bash
sudo systemctl daemon-reload
sudo systemctl enable rustberry-poe-monitor.service
sudo systemctl start rustberry-poe-monitor.service
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
