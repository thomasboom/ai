#!/bin/bash

set -e

INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="airun"
REPO_URL="https://github.com/thomasboom/AIRUN"

echo -e "\n\033[36m  AI Agent Launcher Installer\033[0m\n"
echo "  Launch any AI CLI in seconds"
echo ""

if [ -d "$INSTALL_DIR" ]; then
    echo "  Install directory exists: $INSTALL_DIR"
else
    echo "  Creating install directory: $INSTALL_DIR"
    mkdir -p "$INSTALL_DIR"
fi

if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
    echo "  Building from source..."
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    PARENT_DIR="$(dirname "$SCRIPT_DIR")"
    
    if [ -f "$PARENT_DIR/AIRUN/Cargo.toml" ]; then
        cd "$PARENT_DIR/AIRUN"
        cargo build --release
        cp target/release/$BINARY_NAME "$INSTALL_DIR/$BINARY_NAME"
        echo "  Built and installed successfully!"
    else
        echo "  Source not found. Please clone the repository first."
        exit 1
    fi
else
    echo "  Rust not found. Please install Rust from https://rustup.rs"
    echo "  Then run this installer again."
    exit 1
fi

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "  \033[33mWarning: $INSTALL_DIR is not in your PATH\033[0m"
    echo ""
    echo "  Add this to your shell config (~/.bashrc or ~/.zshrc):"
    echo ""
    echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
    echo "  Then run: source ~/.bashrc (or source ~/.zshrc)"
fi

echo ""
echo "  \033[32mInstallation complete!\033[0m"
echo "  Run 'airun' to launch AIRun."
echo ""
