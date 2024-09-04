#!/bin/bash
set -e

systemd-analyze security clamav-desktop-daemon > ./dev/security.deb.systemd.md
