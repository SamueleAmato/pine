#!/bin/bash

# Create .pine directory in the home folder
mkdir -p ~/.pine

# Create .pine/note and .pine/notebook directories
mkdir -p ~/.pine/note
mkdir -p ~/.pine/notebook

# Create .pine/config.json with {"editor":"vi"} content
echo '{"editor":"vi"}' > ~/.pine/config.json

# Print installation completed message
echo "Installation completed"
