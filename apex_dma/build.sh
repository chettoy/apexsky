#!/usr/bin/env bash

# Causes bash to print each command before executing it
set -x

# Exit immediately when a command fails
set -eo pipefail
 
mkdir -p build
cd build
cmake .. && cmake --build .
