#!/bin/bash
set -e

yarn release:deb
# Allow `_apt` user to access the `deb` file
mv ./src-tauri/target/release/bundle/deb/clamav-desktop_0.0.0_amd64.deb /tmp
if dpkg -l | grep -qw twps-desktop; then
  sudo apt remove -y clamav-desktop || true
fi
sudo apt install -y /tmp/clamav-desktop_0.0.0_amd64.deb
