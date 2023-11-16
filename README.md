# RustBerry-PoE-Monitor

![Rust](https://img.shields.io/badge/rust-%23000000.svg?logo=rust&logoColor=white)
![Raspi](https://img.shields.io/badge/Raspberry%20Pi-A22846?logo=Raspberry%20Pi&logoColor=white)

## 🚨 THIS PROJECT IS UNDER DEVELOPMENT 🚨

RustBerry-PoE-Monitor is a Rust-based monitoring and control tool for the Raspberry Pi, specifically designed for use with the Waveshare PoE HAT (B). 

![Example image](/docs/IMG_3878.png)

This tool provides real-time monitoring of your Raspberry Pi's system statistics, including IP address, memory usage, CPU load, and more, displayed on the PoE HAT's OLED display. Additionally, it offers intelligent fan control to maintain optimal operating temperatures.

## Features

- **Minimal** resource usage
- Developed in Rust for memory safety
- Display real-time system statistics (IP address, memory, CPU usage, etc.) on the PoE HAT's OLED screen
- Automatic fan control based on the CPU temperature [ _Coming Soon_ ]

## Building

### Prerequisites
For building from my PC to Rasberry Pi I'm using [cross](https://github.com/cross-rs/cross)

### Building for Raspberry Pi
```bash
 cross build --target=aarch64-unknown-linux-gnu
```

## Running

Just run the binary file
```bash
./rustberry-poe-monitor
```