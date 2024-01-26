#!/usr/bin/env bash

# Causes bash to print each command before executing it
set -x

# Exit immediately when a command fails
set -eo pipefail

# Determine the script's directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

BUILD_DIR="$SCRIPT_DIR/build"

cd "$BUILD_DIR"
sudo env WAYLAND_DISPLAY="$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" XDG_RUNTIME_DIR=/run/user/0 ./apex_dma
