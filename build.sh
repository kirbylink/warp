#!/bin/bash
set -e

# === CONFIGURATION ===
XCODE_DMG="/home/developer/Downloads/Command_Line_Tools_for_Xcode_16.4.dmg"

# === TARGET DIRECTORY (default: $HOME) ===
BASE_DIR="${1:-$HOME}"

OSXCROSS_DIR="$BASE_DIR/osxcross"
ZIG_VERSION="0.15.2"
ZIG_DIR="$BASE_DIR/.local/zig"

echo "Using base directory: $BASE_DIR"
mkdir -p "$BASE_DIR"

# === 1. Install Rust (user-level) ===
if ! command -v rustup &> /dev/null; then
  echo "Installing rustup..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain=1.93.1 -y
  export PATH="$HOME/.cargo/bin:$PATH"
  rustup target add aarch64-unknown-linux-gnu
  rustup target add x86_64-apple-darwin
  rustup target add aarch64-apple-darwin
  rustup target add x86_64-pc-windows-gnu
  rustup target add aarch64-pc-windows-gnullvm
fi

# === 2. Install cargo-zigbuild ===
echo "Installing cargo-zigbuild..."
cargo install cargo-zigbuild

# === 3. Download Zig compiler ===
echo "Downloading Zig $ZIG_VERSION..."
mkdir -p "$ZIG_DIR"
cd "$ZIG_DIR"
curl -LO "https://ziglang.org/download/$ZIG_VERSION/zig-x86_64-linux-$ZIG_VERSION.tar.xz"
tar -xf "zig-x86_64-linux-$ZIG_VERSION.tar.xz"
rm "zig-x86_64-linux-$ZIG_VERSION.tar.xz"

export PATH="$PATH:$ZIG_DIR/zig-linux-x86_64-$ZIG_VERSION"

# === 4. Clone and build warp ===
cd "$BASE_DIR"
echo "Cloning warp..."
git clone https://github.com/kirbylink/warp.git
cd warp
echo "Building warp..."
make

echo "Bundle warp-packer files..."
mkdir target/bundle
cp target/aarch64-apple-darwin/release/warp-packer target/bundle/macos-aarch64.warp-packer
cp target/x86_64-apple-darwin/release/warp-packer target/bundle/macos-x64.warp-packer
cp target/universal2-apple-darwin/release/warp-packer target/bundle/macos-universal.warp-packer
cp target/aarch64-pc-windows-gnullvm/release/warp-packer.exe target/bundle/windows-aarch64.warp-packer.exe
cp target/x86_64-pc-windows-gnu/release/warp-packer.exe target/bundle/windows-x64.warp-packer.exe
cp target/aarch64-unknown-linux-gnu/release/warp-packer target/bundle/linux-aarch64.warp-packer
cp target/x86_64-unknown-linux-gnu/release/warp-packer target/bundle/linux-x64.warp-packer

echo "Build finished."
