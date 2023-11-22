#!/bin/bash
LATEST_RELEAST_URL="https://github.com/jackra1n/RustBerry-PoE-Monitor/releases/latest/download/rustberry-poe-monitor"

# Check if script is run as root
if [ "$EUID" -ne 0 ]
  then echo "Please run as root"
  exit
fi

# Download latest binary from github and place it in /usr/local/bin
echo "Downloading latest binary from github"
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

