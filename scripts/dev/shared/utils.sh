#!/bin/bash

function must_be_root() {
    if [ "$(id -u)" -ne 0 ]; then
        log.error "This script must be run as root."

        exit 1
    fi
}
