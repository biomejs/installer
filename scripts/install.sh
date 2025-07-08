#!/bin/env bash
# Biome Installer Bootstrap Script for macOS and Linux
# https://github.com/biomejs/installer

set -euo pipefail

function get_os() {
  case "$(uname -s)" in
    Linux*)   echo "linux" ;;
    Darwin*)  echo "darwin" ;;
    CYGWIN*)  echo "windows" ;;
    *) echo "Unsupported platform"; exit 1;;
  esac
}

function get_arch() {
  case "$(uname -m)" in
    x86_64) echo "x86_64" ;;
    aarch64 | arm64) echo "aarch64" ;;
    *) echo "Unsupported architecture"; exit 1;;
  esac
}

function download_installer() {
    # Check if curl is installed
    if ! command -v curl &> /dev/null; then
        echo "curl is not installed. Please install curl and try again."
        exit 1
    fi

    local os
    local arch
    local extension=""

    os=$(get_os)
    arch=$(get_arch)
    extension=""

    if [[ "$os" == "windows" ]]; then
        extension=".exe"
    fi

    local url="https://github.com/biomejs/installer/releases/latest/download/biome-installer-${os}-${arch}${extension}"

    # Download the installer to a temporary file
    local temp_file
    temp_file=$(mktemp /tmp/biome-installer.XXXXXX)
    
    if ! curl -fsSL "$url" -o "$temp_file"; then
        echo "Failed to download the installer."
        exit 1
    fi

    # Make the downloaded file executable
    chmod +x "$temp_file"

    "$temp_file" "$@"
}

function main() {
    download_installer "$@"
}

main "$@"