#!/bin/bash

# https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux
echo "
  ╭―――――――――――――――――――――――――――――――――――――╮
  │ 💾 Installing Tauri dependencies... │
  ╰―――――――――――――――――――――――――――――――――――――╯
"
sudo apt-get update
sudo apt-get install -y \
  libwebkit2gtk-4.0-dev \
  build-essential \
  curl \
  wget \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
if [ ! -z "${CI}" ]; then
  sudo apt-get install -y libdbus-1-dev pkg-config
fi
