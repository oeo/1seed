#!/usr/bin/env bash
set -euo pipefail

# 1seed installer
# Usage: curl -fsSL https://raw.githubusercontent.com/oeo/1seed/master/install.sh | bash

REPO="oeo/1seed"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # no color

info() {
    echo -e "${GREEN}==>${NC} $*"
}

error() {
    echo -e "${RED}Error:${NC} $*" >&2
    exit 1
}

warn() {
    echo -e "${YELLOW}Warning:${NC} $*" >&2
}

# detect OS and architecture
detect_platform() {
    local os arch

    case "$(uname -s)" in
        Linux)     os="linux" ;;
        Darwin)    os="darwin" ;;
        MINGW*|MSYS*|CYGWIN*) os="windows" ;;
        *)         error "Unsupported OS: $(uname -s)" ;;
    esac

    case "$(uname -m)" in
        x86_64|amd64) arch="amd64" ;;
        aarch64|arm64) arch="arm64" ;;
        *)            error "Unsupported architecture: $(uname -m)" ;;
    esac

    echo "${os}-${arch}"
}

# get latest release version
get_latest_version() {
    curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name":' \
        | sed -E 's/.*"([^"]+)".*/\1/'
}

main() {
    info "Installing 1seed..."

    local platform version asset_name archive_ext
    platform=$(detect_platform)
    
    info "Detected platform: ${platform}"

    # determine archive extension
    if [[ $platform == windows* ]]; then
        archive_ext="zip"
    else
        archive_ext="tar.gz"
    fi

    # get latest version
    info "Fetching latest release..."
    version=$(get_latest_version)
    
    if [[ -z $version ]]; then
        error "Could not determine latest version"
    fi

    info "Latest version: ${version}"

    # construct download URL
    asset_name="1seed-${platform}.${archive_ext}"
    download_url="https://github.com/${REPO}/releases/download/${version}/${asset_name}"

    info "Downloading from: ${download_url}"

    # create temp directory
    tmp_dir=$(mktemp -d)
    trap 'rm -rf "$tmp_dir"' EXIT

    # download
    if ! curl -fsSL -o "${tmp_dir}/${asset_name}" "${download_url}"; then
        error "Failed to download ${asset_name}"
    fi

    # extract
    info "Extracting..."
    cd "$tmp_dir"

    if [[ $archive_ext == "tar.gz" ]]; then
        tar xzf "${asset_name}"
    else
        unzip -q "${asset_name}"
    fi

    # find the binary (archive contains file named like "1seed-darwin-arm64")
    binary_name="1seed"
    [[ $platform == windows* ]] && binary_name="1seed.exe"

    extracted_name="1seed-${platform}"
    [[ $platform == windows* ]] && extracted_name="${extracted_name}.exe"

    if [[ -f "${extracted_name}" ]]; then
        binary_path="${extracted_name}"
    elif [[ -f "${binary_name}" ]]; then
        binary_path="${binary_name}"
    else
        error "Could not find binary in archive"
    fi

    # create install directory if needed
    mkdir -p "$INSTALL_DIR"

    # install
    info "Installing to ${INSTALL_DIR}/${binary_name}"
    cp "$binary_path" "${INSTALL_DIR}/${binary_name}"
    chmod +x "${INSTALL_DIR}/${binary_name}"

    # check if in PATH
    if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
        warn "${INSTALL_DIR} is not in your PATH"
        info "Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
        echo ""
        echo "    export PATH=\"\$PATH:${INSTALL_DIR}\""
        echo ""
    fi

    info "Installation complete!"
    info "Run '1seed --help' to get started"
}

main "$@"
