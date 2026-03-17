#!/usr/bin/env bash
set -euo pipefail

# === TARGET DIRECTORY (default: $HOME) ===
BASE_DIR="${1:-$HOME}"

ZIG_VERSION="0.15.2"
ZIG_DIR="$BASE_DIR/.local/zig"
ZIG_INSTALL_DIR="$ZIG_DIR/zig-x86_64-linux-$ZIG_VERSION"

echo "Using base directory: $BASE_DIR"
mkdir -p "$BASE_DIR"

# clean previous build
rm -rf "$BASE_DIR/warp"

# === 1. Install Rust (user-level) ===
if ! command -v $BASE_DIR/.cargo/bin/rustup >/dev/null 2>&1; then
  echo "Installing rustup..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain=1.93.1 -y
fi

export PATH="$BASE_DIR/.cargo/bin:$PATH"

echo "Ensuring required Rust targets..."
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-gnu
rustup target add aarch64-pc-windows-gnullvm

# === 2. Install cargo-zigbuild (only if missing) ===
if ! command -v cargo-zigbuild >/dev/null 2>&1; then
  echo "Installing cargo-zigbuild..."
  cargo install cargo-zigbuild
else
  echo "cargo-zigbuild already installed."
fi

# === 3. Download Zig compiler (only if missing) ===
if [ ! -d "$ZIG_INSTALL_DIR" ]; then
  echo "Downloading Zig $ZIG_VERSION..."
  mkdir -p "$ZIG_DIR"
  cd "$ZIG_DIR"

  curl -LO "https://ziglang.org/download/$ZIG_VERSION/zig-x86_64-linux-$ZIG_VERSION.tar.xz"
  tar -xf "zig-x86_64-linux-$ZIG_VERSION.tar.xz"
  rm "zig-x86_64-linux-$ZIG_VERSION.tar.xz"
else
  echo "Zig $ZIG_VERSION already installed."
fi

export PATH="$PATH:$ZIG_INSTALL_DIR"

# === 4. Clone and build warp ===
cd "$BASE_DIR"

echo "Cloning warp..."
git clone --depth 1 https://github.com/kirbylink/warp.git
cd warp

echo "Building warp..."
make

echo "Bundling warp-packer files..."
mkdir -p target/bundle

cp target/aarch64-apple-darwin/release/warp-packer target/bundle/macos-aarch64.warp-packer
cp target/x86_64-apple-darwin/release/warp-packer target/bundle/macos-x64.warp-packer
cp target/universal2-apple-darwin/release/warp-packer target/bundle/macos-universal.warp-packer
cp target/aarch64-pc-windows-gnullvm/release/warp-packer.exe target/bundle/windows-aarch64.warp-packer.exe
cp target/x86_64-pc-windows-gnu/release/warp-packer.exe target/bundle/windows-x64.warp-packer.exe
cp target/aarch64-unknown-linux-gnu/release/warp-packer target/bundle/linux-aarch64.warp-packer
cp target/x86_64-unknown-linux-gnu/release/warp-packer target/bundle/linux-x64.warp-packer

echo "Build finished."