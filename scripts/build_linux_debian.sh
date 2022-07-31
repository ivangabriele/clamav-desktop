#!/bin/bash

if [ -z "${CI}" ]; then
  echo "
  ╭―――――――――――――――――――――――――――――――――――╮
  │ 📦 Initializing Git submodules... │
  ╰―――――――――――――――――――――――――――――――――――╯
  "
  git submodule update --init --recursive
  git submodule
fi

echo "
  ╭―――――――――――――――――――――――――――――――――――――――╮
  │ 📂 Entering Clamav build directory... │
  ╰―――――――――――――――――――――――――――――――――――――――╯
"
mkdir ./clamav/build
cd ./clamav/build
echo "Working directory: ${PWD}"

# https://docs.clamav.net/manual/Installing/Installing-from-source-Unix.html#ubuntu--debian
echo "
  ╭――――――――――――――――――――――――――――――――――――――――――――╮
  │ 💾 Installing Clamav build dependencies... │
  ╰――――――――――――――――――――――――――――――――――――――――――――╯
"
sudo apt-get update
sudo apt-get install -y \
  `# install tools` \
  gcc make pkg-config python3 python3-pip python3-pytest valgrind \
  `# install clamav dependencies` \
  check libbz2-dev libcurl4-openssl-dev libjson-c-dev libmilter-dev \
  libncurses5-dev libpcre2-dev libssl-dev libxml2-dev zlib1g-dev
python3 -m pip install cmake
if [ ! -z "${CI}" ]; then
  sudo apt-get install -y libdbus-1-dev pkg-config
fi

echo "
  ╭―――――――――――――――――――――――╮
  │ 🔨 Building Clamav... │
  ╰―――――――――――――――――――――――╯
"
cmake ..
cmake --build .

echo "
  ╭――――――――――――――――――――――――――――╮
  │ 🧪 Testing Clamav build... │
  ╰――――――――――――――――――――――――――――╯
"
ctest

echo "
  ╭――――――――――――――――――――――――――――――――――――――╮
  │ 📁 Leaving Clamav build directory... │
  ╰――――――――――――――――――――――――――――――――――――――╯
"

cd ../..
echo "Working directory: ${PWD}"

echo "
  ╭――――――――――――――――――――――――╮
  │ 📝 Copying binaries... │
  ╰――――――――――――――――――――――――╯
"
if [ -z "${CI}" ]; then
  echo "📦 Initializing Git submodules..."
  git submodule update --init --recursive
fi
mkdirp ./bin
cp ./clamav/build/clamd/clamd ./bin/clamd
cp ./clamav/build/clamdscan/clamdscan ./bin/clamdscan
cp ./clamav/build/clamdtop/clamdtop ./bin/clamdtop
cp ./clamav/build/clamscan/clamscan ./bin/clamscan
cp ./clamav/build/freshclam/freshclam ./bin/freshclam

echo "
  ╭――――――――――――――――――――――――――――――――――――――――――――╮
  │ 💾 Installing Clamav build dependencies... │
  ╰――――――――――――――――――――――――――――――――――――――――――――╯
"
yarn tauri build
