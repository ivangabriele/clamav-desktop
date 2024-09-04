#!/bin/bash
set -e

# Build daemon
yarn build:daemon

# Stop the service if it's running
if systemctl is-active --quiet clamav-desktop-daemon.service; then
    sudo systemctl stop clamav-desktop-daemon.service
fi

# Remove service file & binary
sudo rm -f /usr/lib/systemd/system/clamav-desktop-daemon.service
sudo rm -f /usr/bin/clamav-desktop-daemon

# Reload systemd manager configuration
sudo systemctl daemon-reload
