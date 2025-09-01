#!/bin/bash
# Build script for Ubuntu Linux

echo "Building for Linux..."

# Install Rust if not present
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
fi

# Build the project
cargo build --release

echo "Build complete! Binary is at: target/release/server"