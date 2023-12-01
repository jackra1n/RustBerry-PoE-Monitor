#!/bin/bash

# Check if script is run as root
if [ "$EUID" -ne 0 ]
  then echo "Please run as root"
  exit
fi

# Only run on aarch64 or armv7l
ARCH=$(uname -m)
echo "Architecture: $ARCH"
if [ "$ARCH" != "aarch64" ] && [ "$ARCH" != "armv7l" ]; then
    echo "This script is only intended to run on aarch64 or armv7l"
    exit 1
fi

# Check if running on a Raspberry Pi
if ! grep -q "Raspberry Pi" /proc/device-tree/model; then
    echo "This script is only intended to run on a Raspberry Pi"
    exit 1
fi

# Enable I2C
echo "Enabling I2C"
sudo raspi-config nonint do_i2c 0

# Set download url based on architecture
LATEST_RELEAST_URL="https://github.com/jackra1n/RustBerry-PoE-Monitor/releases/latest/download/rustberry-poe-monitor-$ARCH"

# Download latest binary from github and place it in /usr/local/bin
echo "Downloading latest $ARCH binary from github"
curl -sSL $LATEST_RELEAST_URL -o /usr/local/bin/rustberry-poe-monitor

# Make binary executable
chmod +x /usr/local/bin/rustberry-poe-monitor

# Create systemd service
echo "Creating systemd service"
sudo cat <<EOF > /etc/systemd/system/rustberry-poe-monitor.service
[Unit]
Description=RustBerry PoE Monitor
After=network.target

[Service]
ExecStart=/usr/local/bin/rustberry-poe-monitor
Restart=always
RestartSec=30

[Install]
WantedBy=multi-user.target
EOF

# Enable service
echo "Enabling systemd service"
sudo systemctl daemon-reload
sudo systemctl enable rustberry-poe-monitor.service
sudo systemctl start rustberry-poe-monitor.service

echo "Installation complete"

