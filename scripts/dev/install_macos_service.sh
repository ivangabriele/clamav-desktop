#!/bin/bash
set -e

export SCRIPT_NAME="$(basename "$0")"
SCRIPT_PATH="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
source "$SCRIPT_PATH/shared/constants.sh"
source "$SCRIPT_PATH/shared/log.sh"
source "$SCRIPT_PATH/shared/utils.sh"

must_be_root

log.info "Installing macOS service..."
cp "./src-tauri/embeds/macos/${MACOS_SERVICE_NAME}.plist" "/Library/LaunchDaemons/${MACOS_SERVICE_NAME}.plist"
cp "./daemon/target/debug/${UNIX_BINARY_NAME}" "/usr/local/bin/${UNIX_BINARY_NAME}"
log.success "macOS service installed successfully."

log.info "Starting macOS service..."
launchctl load "/Library/LaunchDaemons/${MACOS_SERVICE_NAME}.plist"
launchctl start "${MACOS_SERVICE_NAME}"
log.success "macOS service started successfully."

"$SCRIPT_PATH/log_macos_service.sh"
