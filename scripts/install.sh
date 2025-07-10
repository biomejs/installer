#!/bin/env bash
##############################################################################
#                                                                            #
#   ____  _                           ___           _        _ _             #
#  | __ )(_) ___  _ __ ___   ___     |_ _|_ __  ___| |_ __ _| | | ___ _ __   #
#  |  _ \| |/ _ \| '_ ` _ \ / _ \     | || '_ \/ __| __/ _` | | |/ _ \ '__|  #
#  | |_) | | (_) | | | | | |  __/     | || | | \__ \ || (_| | | |  __/ |     #
#  |____/|_|\___/|_| |_| |_|\___|    |___|_| |_|___/\__\__,_|_|_|\___|_|     #
#                                                                            #
#  Biome Installer Bootstrapper                                              #
#                                                                            #
#  This script downloads and runs the Biome installer for the current os     #
#  and architecture. It is designed to be run on Linux and macOS systems.    #
#  It requires curl to be installed.                                         #
#                                                                            #
#  License: MIT or Apache-2.0                                                #
#                                                                            #
#  Copyright (c) 2025 Nicolas Hedger                                         #
#                                                                            #
##############################################################################

set -euo pipefail

# Removes the temporary file specified by $temp_file if it exists.
function cleanup() {
  [[ -n "${temp_file:-}" && -f "$temp_file" ]] && rm -f "$temp_file"
}

# Trap to ensure cleanup is called on script exit
trap cleanup EXIT

# Determines the operating system
function get_os() {
  case "$(uname -s)" in
    Linux*)   echo "linux" ;;
    Darwin*)  echo "darwin" ;;
    *) echo "Unsupported platform: $(uname -s)" >&2; exit 1;;
  esac
}

# Determines the architecture
function get_arch() {
  case "$(uname -m)" in
    amd64 | x86_64) echo "x86_64" ;;
    aarch64 | arm64) echo "aarch64" ;;
    *) echo "Unsupported architecture: $(uname -m)" >&2; exit 1;;
  esac
}

# Downloads the installer and runs it
function main() {
    if ! command -v curl &> /dev/null; then
        echo "Error: curl is required but not installed" >&2
        exit 1
    fi

    local os arch url

    os=$(get_os)
    arch=$(get_arch)
    temp_file=$(mktemp)
    url="https://github.com/biomejs/installer/releases/latest/download/biome-installer-${os}-${arch}"

    echo "Downloading and running Biome installer for ${os} (${arch})..."
    
    if ! curl -fsSL --retry 3 "$url" -o "$temp_file"; then
        echo "Error: Failed to download installer" >&2
        exit 1
    fi

    chmod +x "$temp_file"

    "$temp_file" "$@"
}

main "$@"