#!/usr/bin/env bash

# Causes bash to print each command before executing it
set -x

# Exit immediately when a command fails
set -eo pipefail

# Print Rust version
cargo --version

# Determine the script's directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create build directory
BUILD_DIR="$SCRIPT_DIR/build"
mkdir -p "$BUILD_DIR"

# Build kmod for memflow-kvm
cd "$SCRIPT_DIR/lib/memflow-kvm"
set +e
make && cp "$SCRIPT_DIR/lib/memflow-kvm/build/memflow.ko" "$BUILD_DIR/"
set -e

# Build release version of apexsky
cd "$SCRIPT_DIR/apexsky"

cargo build --release
cd "$SCRIPT_DIR"

# Build CMake project
cd "$BUILD_DIR"
cmake .. && cmake --build .
