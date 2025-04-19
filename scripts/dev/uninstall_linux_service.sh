#!/bin/bash
set -e

# Stop the service if it's running
if systemctl is-active --quiet clamav-desktop-daemon.service; then
    sudo systemctl stop clamav-desktop-daemon.service
fi

# Remove service file & binary
# Note:
# `sudo systemctl enable` creates a symlink in:
# `/etc/systemd/system/multi-user.target.wants/clamav-desktop-daemon.service`,
# so we also have to remove that symlink.
sudo rm -f /etc/systemd/system/multi-user.target.wants/clamav-desktop-daemon.service
sudo rm -f /usr/lib/systemd/system/clamav-desktop-daemon.service
sudo rm -f /usr/bin/clamav-desktop-daemon

# Reload systemd manager configuration
sudo systemctl daemon-reload
