#!/bin/bash
set -e

function log_error_and_exit {
    echo -e "\033[0;31m[install_debian_requirements] Error: $1\033[0m" 1>&2
    exit 1
}

if [ "$(id -u)" -ne 0 ]; then
    log_error_and_exit "Please run as root."
fi

################################################################################
# Detect OS and version

if [ -f /etc/os-release ]; then
    # Source the os-release file to get the necessary information
    . /etc/os-release

    OS_NAME=$NAME
    OS_VERSION=$VERSION_ID

    if [[ "${OS_NAME}" == "Debian" ]]; then
        if [[ "${OS_VERSION%%.*}" -lt 12 ]]; then
            log_error_and_exit "Debian v${OS_VERSION} is unsupported. Required: Debian 12 or later."
        fi
    elif [[ "${OS_NAME}" == "Ubuntu" ]]; then
        if [[ "${OS_VERSION%%.*}" -lt 22 ]]; then
            log_error_and_exit "Ubuntu v${OS_VERSION} is unsupported. Required: Ubuntu 22 or later."
        fi
    else
        log_error_and_exit "Unknown OS: ${NAME}, version: ${OS_VERSION}."
    fi

else
    log_error_and_exit "`/etc/os-release` file not found."
fi

################################################################################
# Update package list

apt-get update

################################################################################
# Install Tauri build dependencies
# https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux
apt install -y \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

if [[ "${OS_VERSION%%.*}" -ge 24 ]]; then
    echo "deb http://archive.ubuntu.com/ubuntu jammy main" | tee /etc/apt/sources.list.d/jammy.list
    apt-get update
    apt-get install -y libwebkit2gtk-4.0-dev
    rm /etc/apt/sources.list.d/jammy.list
    apt-get update
else
    apt install -y libwebkit2gtk-4.0-dev
fi

################################################################################
# Install ClamAV build dependencies
# https://docs.clamav.net/manual/Installing/Installing-from-source-Unix.html#ubuntu--debian

apt-get install -y \
  `# install tools` \
  gcc make pkg-config python3 python3-pip python3-pytest valgrind cmake \
  `# install ClamAV dependencies` \
  check libbz2-dev libcurl4-openssl-dev libjson-c-dev libmilter-dev \
  libncurses5-dev libpcre2-dev libssl-dev libxml2-dev zlib1g-dev \
  `# install ninja for faster builds` \
  ninja-build \
  `# install doc-missing dependencies` \
  javascriptcoregtk-4.0

################################################################################
# Install custom dependencies

# apt-get install -y \
#     javascriptcoregtk-4.0 \
#     javascriptcoregtk-4.0-dev \
#     libsoup-2.4
