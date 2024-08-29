#!/bin/bash
set -e

# https://docs.clamav.net/manual/Installing/Installing-from-source-Unix.html#ubuntu--debian

apt-get update && apt-get install -y \
  `# install tools` \
  gcc make pkg-config python3 python3-pip python3-pytest valgrind cmake \
  `# install clamav dependencies` \
  check libbz2-dev libcurl4-openssl-dev libjson-c-dev libmilter-dev \
  libncurses5-dev libpcre2-dev libssl-dev libxml2-dev zlib1g-dev \
  `# install ninja for faster builds` \
  ninja-build
