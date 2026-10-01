#!/usr/bin/env bash
# FileZen Universal Installer
# Enables anyone to install or update FileZen with a single command:
#   curl -fsSL https://raw.githubusercontent.com/gurbaxani/filezen/trunk/install.sh | bash

set -euo pipefail

REPO="gurbaxani/filezen"
BINARY_NAME="filezen"

# Styling
BOLD="$(tput bold 2>/dev/null || echo '')"
GREEN="$(tput setaf 2 2>/dev/null || echo '')"
YELLOW="$(tput setaf 3 2>/dev/null || echo '')"
RED="$(tput setaf 1 2>/dev/null || echo '')"
CYAN="$(tput setaf 6 2>/dev/null || echo '')"
RESET="$(tput sgr0 2>/dev/null || echo '')"

info() {
    printf "${CYAN}${BOLD}==>${RESET} ${BOLD}%s${RESET}\n" "$*"
}

success() {
    printf "${GREEN}${BOLD}✔${RESET} %s\n" "$*"
}

warn() {
    printf "${YELLOW}${BOLD}Warning:${RESET} %s\n" "$*" >&2
}

error() {
    printf "${RED}${BOLD}Error:${RESET} %s\n" "$*" >&2
    exit 1
}

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        case "$ARCH" in
            x86_64)  TARGET="x86_64-unknown-linux-musl" ;;
            aarch64) TARGET="aarch64-unknown-linux-musl" ;;
            arm64)   TARGET="aarch64-unknown-linux-musl" ;;
            *)       error "Unsupported architecture: $ARCH on Linux" ;;
        esac
        ;;
    Darwin)
        case "$ARCH" in
            x86_64)  TARGET="x86_64-apple-darwin" ;;
            arm64)   TARGET="aarch64-apple-darwin" ;;
            *)       error "Unsupported architecture: $ARCH on macOS" ;;
        esac
        ;;
    *)
        error "Unsupported operating system: $OS. FileZen currently supports Linux and macOS via this installer."
        ;;
esac

info "Detected platform: ${OS} (${ARCH})"

# Determine installation directory
if [ -w "/usr/local/bin" ] || [ "$(id -u)" -eq 0 ]; then
    INSTALL_DIR="/usr/local/bin"
else
    INSTALL_DIR="${HOME}/.local/bin"
    mkdir -p "${INSTALL_DIR}"
fi

info "Installing FileZen into ${INSTALL_DIR}..."

TMP_DIR="$(mktemp -d)"
cleanup() {
    rm -rf "${TMP_DIR}"
}
trap cleanup EXIT

RELEASE_URL="https://github.com/${REPO}/releases/latest/download/filezen-${TARGET}.tar.gz"

DOWNLOADED=0
if command -v curl >/dev/null 2>&1; then
    if curl -fsSL "${RELEASE_URL}" -o "${TMP_DIR}/filezen.tar.gz" 2>/dev/null; then
        if tar -xzf "${TMP_DIR}/filezen.tar.gz" -C "${TMP_DIR}" 2>/dev/null; then
            DOWNLOADED=1
        fi
    fi
elif command -v wget >/dev/null 2>&1; then
    if wget -qO "${TMP_DIR}/filezen.tar.gz" "${RELEASE_URL}" 2>/dev/null; then
        if tar -xzf "${TMP_DIR}/filezen.tar.gz" -C "${TMP_DIR}" 2>/dev/null; then
            DOWNLOADED=1
        fi
    fi
fi

if [ "${DOWNLOADED}" -eq 1 ] && [ -f "${TMP_DIR}/${BINARY_NAME}" ]; then
    install -m 755 "${TMP_DIR}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
elif command -v cargo >/dev/null 2>&1; then
    warn "Pre-built binary release asset not found for ${TARGET}. Falling back to cargo install..."
    # Always pass --force to cleanly overwrite/upgrade existing binary
    cargo install --git "https://github.com/${REPO}" --root "${INSTALL_DIR}/.." --force
else
    error "Could not download pre-compiled binary for ${TARGET} and cargo is not installed. Please visit https://github.com/${REPO}/releases to download your platform's binary directly."
fi

# Verify installation
if ! command -v "${BINARY_NAME}" >/dev/null 2>&1; then
    if [ -f "${INSTALL_DIR}/${BINARY_NAME}" ]; then
        warn "'${INSTALL_DIR}' is not in your PATH."
        warn "Add the following line to your shell configuration file (~/.bashrc or ~/.zshrc):"
        printf "\n    export PATH=\"%s:\$PATH\"\n\n" "${INSTALL_DIR}"
    fi
fi

printf "\n"
success "FileZen has been successfully installed on your machine!"
printf "\n"
printf "  ${BOLD}Try running it now:${RESET}\n"
printf "    ${CYAN}%s${RESET}                  # Organizes your current directory\n" "${BINARY_NAME}"
printf "    ${CYAN}%s ~/Downloads${RESET}      # Organizes your Downloads folder\n" "${BINARY_NAME}"
printf "    ${CYAN}%s --help${RESET}          # Shows all available options\n\n" "${BINARY_NAME}"
