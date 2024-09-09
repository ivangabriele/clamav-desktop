#!/bin/bash
set -e

export SCRIPT_NAME="$(basename "$0")"
SCRIPT_PATH="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
source "$SCRIPT_PATH/shared/constants.sh"
source "$SCRIPT_PATH/shared/log.sh"
source "$SCRIPT_PATH/shared/utils.sh"

must_be_root

log.log "Getting macOS service status..."
launchctl list | grep com.clamav-desktop.daemon

# log.log "Tailing macOS service logs... (Press Ctrl+C to stop)"
# log stream --predicate "process == \"${UNIX_BINARY_NAME}\"" --info
if [ -f "${UNIX_STDOUT_LOG_PATH}" ] && [ -f "${UNIX_STDERR_LOG_PATH}" ]; then
    log.log "Tailing macOS service logs... (Press Ctrl+C to stop)"
    tail -f "$UNIX_STDOUT_LOG_PATH" "$UNIX_STDERR_LOG_PATH"
else
    if [ ! -f "${UNIX_STDOUT_LOG_PATH}" ]; then
        log.error "Log file '${UNIX_STDERR_LOG_PATH}' does not exist."
    fi
    if [ ! -f "${UNIX_STDERR_LOG_PATH}" ]; then
        log.error "Error log file '${UNIX_STDERR_LOG_PATH}' does not exist."
    fi
fi
