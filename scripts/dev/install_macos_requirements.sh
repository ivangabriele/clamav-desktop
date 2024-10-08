#!/bin/bash
set -e

# https://docs.clamav.net/manual/Installing/Installing-from-source-Unix.html#macos

brew update

packages=(
  # install tools
  python3 cmake
  # install clamav dependencies
  bzip2 check curl-openssl json-c libxml2 ncurses openssl@1.1 pcre2 zlib
)
for item in "${packages[@]}"; do
  brew install $item || true; brew upgrade $item || brew upgrade $item
done

# Install Python packages withing a virtual environment
# Original command: `python3 -m pip install --user cmake pytest``
python3 -m venv venv
source venv/bin/activate
python3 -m pip install cmake pytest

# Install Ninja for faster builds
brew install ninja
