#!/bin/bash
set -e

systemctl status --lines=0 clamav-desktop-daemon.service
journalctl -u clamav-desktop-daemon.service -f
