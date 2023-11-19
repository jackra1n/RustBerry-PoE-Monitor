# 🍇 RustBerry-PoE-Monitor 🖥️

[![Build](https://github.com/jackra1n/RustBerry-PoE-Monitor/actions/workflows/build.yaml/badge.svg)](https://github.com/jackra1n/RustBerry-PoE-Monitor/actions/workflows/build.yaml)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?logo=rust&logoColor=white)
![Raspi](https://img.shields.io/badge/Raspberry%20Pi-A22846?logo=Raspberry%20Pi&logoColor=white)

## 🚨 THIS PROJECT IS UNDER DEVELOPMENT 🚨

Some features are not implemented yet and installation is not fully documented.
If you know what you are doing, you can use the latest release and do it your way or wait untill installation is documented.

---

RustBerry-PoE-Monitor is a Rust-based monitoring and control tool for the Raspberry Pi, specifically designed for use with the [Waveshare PoE HAT (B)](https://www.waveshare.com/wiki/PoE_HAT_(B)). 

![Example image](/docs/IMG_3890.webp)

This tool provides real-time monitoring of your Raspberry Pi's system statistics, including IP address, memory usage, CPU load, and more, displayed on the PoE HAT's OLED display. Additionally, it offers intelligent fan control to maintain optimal operating temperatures.

## 🌟 Features

- **Minimal** resource usage
- Developed in Rust for memory safety 🦀
- Display real-time system statistics (IP address, memory, CPU usage, etc.) on the PoE HAT's OLED screen 📊
- Automatic fan control based on the CPU temperature 🌡️

## 🛠️ Building

### Prerequisites
For building from my PC to Rasberry Pi I'm using [cross](https://github.com/cross-rs/cross)

### Building for Raspberry Pi
```bash
 cross build --target=aarch64-unknown-linux-gnu --release
```

## 🏃‍♂️ Running

Just run the binary file
```bash
./rustberry-poe-monitor
```

---

This project is inspired by [klamann/raspi-poe-mon](https://github.com/klamann/raspi-poe-mon) rewritten in Rust for learning purposes and better resource usage.
