#!/bin/bash

# Exit when any command fails:
set -e

mkdir -p ./packages/main/dist
cp -R ./node_modules/ps-list/vendor ./packages/main/dist
