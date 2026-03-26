#!/bin/bash

set -e

INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="airun"

echo -e "\n\033[36m  AI Agent Launcher Uninstaller\033[0m\n"
echo "  Launch any AI CLI in seconds"
echo ""

if [ -f "$INSTALL_DIR/$BINARY_NAME" ]; then
    rm "$INSTALL_DIR/$BINARY_NAME"
    echo "  Removed: $INSTALL_DIR/$BINARY_NAME"
else
    echo "  Binary not found at $INSTALL_DIR/$BINARY_NAME"
fi

echo ""
echo "  \033[32mUninstall complete!\033[0m"
echo ""
