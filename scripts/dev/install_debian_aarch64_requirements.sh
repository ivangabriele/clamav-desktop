#!/bin/bash
set -e

# https://github.com/Cisco-Talos/clamav/blob/main/INSTALL-cross-linux-arm64.md

# apt-get install -y \
#   `# not in docs` \
#   gcc-aarch64-linux-gnu

# Add missing GPG keys
apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 3B4FE6ACC0B21F32
apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 871920D1991BC93C

dpkg --add-architecture arm64
echo  "deb [arch=arm64] http://ports.ubuntu.com/ noble main restricted" >> /etc/apt/sources.list
echo  "deb [arch=arm64] http://ports.ubuntu.com/ noble-updates main restricted" >> /etc/apt/sources.list
echo  "deb [arch=arm64] http://ports.ubuntu.com/ noble universe" >> /etc/apt/sources.list
echo  "deb [arch=arm64] http://ports.ubuntu.com/ noble-updates universe" >> /etc/apt/sources.list
echo  "deb [arch=arm64] http://ports.ubuntu.com/ noble multiverse" >> /etc/apt/sources.list
echo  "deb [arch=arm64] http://ports.ubuntu.com/ noble-updates multiverse" >> /etc/apt/sources.list
echo  "deb [arch=arm64] http://ports.ubuntu.com/ noble-backports main restricted universe multiverse" >> /etc/apt/sources.list

apt-get update
apt-get install -y \
  check:arm64 \
  g++-aarch64-linux-gnu \
  libbz2-dev:arm64 \
  libcurl4-openssl-dev:arm64 \
  libjson-c-dev:arm64 \
  libmilter-dev:arm64 \
  libncurses5-dev:arm64 \
  libpcre2-dev:arm64 \
  libssl-dev:arm64 \
  libxml2-dev:arm64 \
  zlib1g-dev:arm64
