#!/usr/bin/env bash

# Causes bash to print each command before executing it
set -x

# Exit immediately when a command fails
set -eo pipefail
 
rm -rf ./build ../build

cd memflow
cargo clean
cd ..

cd memflow-qemu
cargo clean
cd ..

cd memflow-win32
cargo clean
cd ..

cd apexsky
cargo clean
cd ..

echo Done

