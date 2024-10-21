#!/usr/bin/env bash

# Causes bash to print each command before executing it
set -x

# Exit immediately when a command fails
set -eo pipefail

# Get the directory of the script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

PROJECT_DIR="$(cd "${SCRIPT_DIR}/" && pwd)"

# Remove the build directory in the root if it exists
if [ -d "${PROJECT_DIR}/build" ]; then
    echo "Removing build directory in the root..."
    rm -rf "${PROJECT_DIR}/build"
fi

# Remove the target directory in the root if it exists
if [ -d "${PROJECT_DIR}/target" ]; then
    echo "Removing target directory in the root..."
    rm -rf "${PROJECT_DIR}/target"
fi

echo "Cleanup completed."
