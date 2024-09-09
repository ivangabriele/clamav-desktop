#!/bin/bash
set -e

export SCRIPT_NAME="$(basename "$0")"
SCRIPT_PATH="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
source "$SCRIPT_PATH/shared/constants.sh"
source "$SCRIPT_PATH/shared/log.sh"
source "$SCRIPT_PATH/shared/utils.sh"

must_be_root

if launchctl list | grep -q "${MACOS_SERVICE_NAME}"; then
  log.info "Uninstalling macOS service..."

  # Try to stop the service first
  if sudo launchctl stop "${MACOS_SERVICE_NAME}"; then
    log.info "Service '${MACOS_SERVICE_NAME}' stopped successfully."
  else
    log.warn "Service '${MACOS_SERVICE_NAME}' was not running or failed to stop."
  fi

  # Using `launchctl bootout`` for richer error reporting
  sudo launchctl bootout system "/Library/LaunchDaemons/${MACOS_SERVICE_NAME}.plist" || {
    log.warn "Failed to unload service with bootout, trying unload instead..."
    launchctl unload "/Library/LaunchDaemons/${MACOS_SERVICE_NAME}.plist"
  }

  log.success "macOS service uninstalled successfully."
else
  log.warn "macOS service is not installed."
fi

log.info "Removing macOS service files..."
rm -f "/Library/LaunchDaemons/${MACOS_SERVICE_NAME}.plist"
rm -f "/usr/local/bin/${UNIX_BINARY_NAME}"
log.success "macOS service files removed successfully."
