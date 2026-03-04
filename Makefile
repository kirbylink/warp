all:
	$(MAKE) build

build:
	# --- Build all warp-runner binaries first ---
	# --- Linux x86_64 (Compatible with Debian 10+, Ubuntu 18.04+) ---
	cargo zigbuild -p warp-runner --release --target x86_64-unknown-linux-gnu.2.28

	# --- Linux ARM64 (Compatible with Debian 10+, Ubuntu 18.04+) ---
	cargo zigbuild -p warp-runner --release --target aarch64-unknown-linux-gnu.2.28

	# --- macOS Intel (x86_64) ---
	# No manual SDK or osxcross-strip required
	cargo zigbuild -p warp-runner --release --target x86_64-apple-darwin

	# --- macOS Apple Silicon (ARM64) ---
	cargo zigbuild -p warp-runner --release --target aarch64-apple-darwin
	
	# --- macOS Universal (Intel + ARM64 in one binary) ---
	cargo zigbuild -p warp-runner --release --target universal2-apple-darwin

	# --- Windows x86_64 ---
	# Using zigbuild here too removes the need for a local 'strip' command
	cargo zigbuild -p warp-runner --release --target x86_64-pc-windows-gnu

	# --- Windows ARM64 ---
	cargo zigbuild -p warp-runner --release --target aarch64-pc-windows-gnullvm

	# --- Build warp-packer after all warp-runner builds are complete ---
	# --- Linux x86_64 (Compatible with Debian 10+, Ubuntu 18.04+) ---
	cargo zigbuild -p warp-packer --release --target x86_64-unknown-linux-gnu.2.28

	# --- Linux ARM64 (Compatible with Debian 10+, Ubuntu 18.04+) ---
	cargo zigbuild -p warp-packer --release --target aarch64-unknown-linux-gnu.2.28

	# --- macOS Intel (x86_64) ---
	# No manual SDK or osxcross-strip required
	cargo zigbuild -p warp-packer --release --target x86_64-apple-darwin

	# --- macOS Apple Silicon (ARM64) ---
	# No manual SDK or osxcross-strip required
	cargo zigbuild -p warp-packer --release --target aarch64-apple-darwin
	
	# --- macOS Universal (Intel + ARM64 in one binary) ---
	cargo zigbuild -p warp-packer --release --target universal2-apple-darwin

	# --- Windows x86_64 ---
	# Using zigbuild here too removes the need for a local 'strip' command
	cargo zigbuild -p warp-packer --release --target x86_64-pc-windows-gnu

	# --- Windows ARM64 ---
	cargo zigbuild -p warp-packer --release --target aarch64-pc-windows-gnullvm

clean:
	cargo clean

check:
	$(MAKE) build
	$(MAKE) test

test:
	cargo test

.PHONY: all build clean check test
