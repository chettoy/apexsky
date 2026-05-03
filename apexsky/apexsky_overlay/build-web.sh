#!/bin/bash
# Exit immediately if a command exits with a non-zero status
set -e

# Build for the web target
cargo build --target wasm32-unknown-unknown --no-default-features --features="web-wasm-webgl" --release

# Navigate to the build output dir `apexsky/target/wasm32-unknown-unknown/release/``
cd ../target/wasm32-unknown-unknown/release/

# Generate JS bindings for the browser
wasm-bindgen --target web --out-dir ./apexsky_overlay-web --no-typescript ./apexsky_overlay.wasm

cd apexsky_overlay-web

# Copy the entry HTML file from the source package
cp ../../../../apexsky_overlay/index.html .

# Optimize WASM binary
wasm-opt -O4 -o opt.wasm apexsky_overlay_bg.wasm
rm apexsky_overlay_bg.wasm
mv opt.wasm apexsky_overlay_bg.wasm
