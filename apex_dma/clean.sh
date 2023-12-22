#!/usr/bin/env bash

# Causes bash to print each command before executing it
set -x

# Exit immediately when a command fails
set -eo pipefail

# Get the directory of the script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Go up one level to the project root
PROJECT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

# Remove the build directory in the root if it exists
if [ -d "${PROJECT_DIR}/build" ]; then
    echo "Removing build directory in the root..."
    rm -rf "${PROJECT_DIR}/build"
fi

# Remove the build directory in the apex_dma subdirectory if it exists
if [ -d "${PROJECT_DIR}/apex_dma/build" ]; then
    echo "Removing build directory in apex_dma subdirectory..."
    rm -rf "${PROJECT_DIR}/apex_dma/build"
fi

# Run 'cargo clean' in apexsky directory
echo "Running 'cargo clean' in apexsky..."
cd "${PROJECT_DIR}/apex_dma/apexsky" && cargo clean

# Run 'cargo clean' in memflow subdirectories
MEMFLOW_DIR="${PROJECT_DIR}/apex_dma/memflow_lib"
for subdirectory in "memflow" "memflow-kvm" "memflow-qemu" "memflow-win32"; do
    if [ -d "${MEMFLOW_DIR}/${subdirectory}" ]; then
        echo "Running 'cargo clean' in ${subdirectory}..."
        (cd "${MEMFLOW_DIR}/${subdirectory}" && cargo clean)
    fi
done

# Run 'make clean' in memflow-kvm directory
echo "Running 'make clean' in memflow-kvm dir..."
cd "${PROJECT_DIR}/apex_dma/memflow_lib/memflow-kvm" && make clean

echo "Cleanup completed."
