#!/bin/bash
set -e

# Install binary & service file
sudo cp ./daemon/target/debug/clamav-desktop-daemon /usr/bin/clamav-desktop-daemon
sudo cp ./src-tauri/embeds/deb/clamav-desktop-daemon.service /usr/lib/systemd/system/clamav-desktop-daemon.service

# Reload systemd manager configuration
sudo systemctl daemon-reload

# Enable and start the service
sudo systemctl enable clamav-desktop-daemon.service
sudo systemctl start clamav-desktop-daemon.service

sudo systemctl status --no-pager clamav-desktop-daemon.service
