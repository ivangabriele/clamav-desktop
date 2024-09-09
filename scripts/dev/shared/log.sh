#!/bin/bash

readonly GRAY='\033[2;37m'
readonly BLUE='\033[1;34m'
readonly GREEN='\033[1;32m'
readonly YELLOW='\033[1;33m'
readonly RED='\033[1;31m'
readonly NO_COLOR='\033[0m'

get_caller_name() {
  if [[ -n "$SCRIPT_NAME" ]]; then
    echo "${SCRIPT_NAME%.*}"
  else
    local script_name=$(basename ${BASH_SOURCE[2]})

    echo "${script_name%.*}"
  fi
}

log.log() {
  local caller=$(get_caller_name)

  echo -e "${GRAY}[${caller}] $*${NO_COLOR}"
}

log.info() {
  local caller=$(get_caller_name)

  echo -e "${BLUE}[${caller}] $*${NO_COLOR}"
}

log.success() {
  local caller=$(get_caller_name)

  echo -e "${GREEN}[${caller}] $*${NO_COLOR}"
}

log.warn() {
  local caller=$(get_caller_name)

  echo -e "${YELLOW}[${caller}] $*${NO_COLOR}"
}

log.error() {
  local caller=$(get_caller_name)

  echo -e "${RED}[${caller}] $*${NO_COLOR}"
}
