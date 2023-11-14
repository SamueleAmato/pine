#!/bin/bash

# Determine system architecture
ARCHITECTURE=$(uname -m)

# Define download URL based on system architecture
if [ "$ARCHITECTURE" == "x86_64" ]; then
    DOWNLOAD_URL="https://github.com/SamueleAmato/pine/releases/download/v0.1.0/pine_x86_64.zip"
elif [ "$ARCHITECTURE" == "i686" ]; then
    DOWNLOAD_URL="https://github.com/SamueleAmato/pine/releases/download/v0.1.0/pine_i686.zip"
else
    echo "Unsupported architecture: $ARCHITECTURE"
    exit 1
fi

# Download and extract the appropriate file
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR" || exit 1

wget "$DOWNLOAD_URL"
unzip "$(basename "$DOWNLOAD_URL")" -d "$TEMP_DIR"

# Move the extracted binary to /usr/local/bin
sudo mv pine /usr/local/bin

# Clean up temporary files
rm -rf "$TEMP_DIR"

# Continue with the rest of the installation

# Create .pine directory in the home folder
mkdir -p ~/.pine

# Create .pine/note and .pine/notebook directories
mkdir -p ~/.pine/note
mkdir -p ~/.pine/notebook

# Create .pine/config.json with {"editor":"vi"} content
echo '{"editor":"vi"}' > ~/.pine/config.json

# Print installation completed message
echo "Installation completed"
