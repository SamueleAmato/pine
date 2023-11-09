#!/bin/bash

# Directory paths
PINE_DIR="$HOME/.pine"
NOTEBOOK_DIR="$PINE_DIR/notebook"
CONFIG_DIR="$HOME/.config"

# Create .pine directory
mkdir -p "$PINE_DIR"

# Create notebook and .config directories inside .pine
mkdir -p "$NOTEBOOK_DIR"
mkdir -p "$CONFIG_DIR"

echo "Installation complete. Directories created:"
echo "  - $PINE_DIR"
echo "  - $NOTEBOOK_DIR"
echo "  - $CONFIG_DIR"
