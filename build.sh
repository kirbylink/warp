#!/bin/bash
set -e

# === CONFIGURATION ===
XCODE_DMG="/path/to/Command_Line_Tools_for_Xcode_12.5.1.dmg"

# === TARGET DIRECTORY (default: $HOME) ===
BASE_DIR="${1:-$HOME}"

OSXCROSS_DIR="$BASE_DIR/osxcross"
ZIG_VERSION="0.14.0"
ZIG_DIR="$BASE_DIR/.local/zig"

echo "🛠 Using base directory: $BASE_DIR"
mkdir -p "$BASE_DIR"

# === 1. Install Rust (user-level) ===
if ! command -v rustup &> /dev/null; then
  echo "📦 Installing rustup..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain=1.87.0 -y
  export PATH="$HOME/.cargo/bin:$PATH"
fi

# === 2. Install cargo-zigbuild ===
echo "📦 Installing cargo-zigbuild..."
cargo install cargo-zigbuild

# === 3. Download Zig compiler ===
echo "⬇️ Downloading Zig $ZIG_VERSION..."
mkdir -p "$ZIG_DIR"
cd "$ZIG_DIR"
curl -LO "https://ziglang.org/download/$ZIG_VERSION/zig-linux-x86_64-$ZIG_VERSION.tar.xz"
tar -xf "zig-linux-x86_64-$ZIG_VERSION.tar.xz"
export PATH="$ZIG_DIR/zig-linux-x86_64-$ZIG_VERSION:$PATH"

# === 4. Download and build osxcross ===
echo "⬇️ Cloning osxcross..."
git clone https://github.com/tpoechtrager/osxcross.git "$OSXCROSS_DIR"
cd "$OSXCROSS_DIR"

echo "📦 Extracting macOS SDK..."
./tools/gen_sdk_package_tools_dmg.sh "$XCODE_DMG"

mkdir -p "$OSXCROSS_DIR/tarballs"
cp -v MacOSX11.3.sdk.tar.xz tarballs/

echo "⚙️ Building osxcross toolchain..."
UNATTENDED=yes OSX_VERSION_MIN=11 SDK_VERSION=11.3 ./build.sh
export PATH="$OSXCROSS_DIR/target/bin:$PATH"

# === 5. Clone and build warp ===
cd "$BASE_DIR"
echo "⬇️ Cloning warp..."
git clone https://github.com/kirbylink/warp.git
cd warp
echo "🚀 Building warp..."
make
