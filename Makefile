all:
	$(MAKE) build

build:
	# --- Build all warp-runner binaries first ---

	# Linux x86_64
	cargo zigbuild -p warp-runner --release --target x86_64-unknown-linux-gnu.2.28
	strip target/x86_64-unknown-linux-gnu/release/warp-runner

	# Linux aarch64
	cargo zigbuild -p warp-runner --release --target aarch64-unknown-linux-gnu.2.28
	aarch64-linux-gnu-strip target/aarch64-unknown-linux-gnu/release/warp-runner

	# macOS x86_64
	cargo build -p warp-runner --release --target x86_64-apple-darwin
	x86_64-apple-darwin24.5-strip target/x86_64-apple-darwin/release/warp-runner
	rcodesign sign target/x86_64-apple-darwin/release/warp-runner

	# macOS ARM64
	cargo build -p warp-runner --release --target aarch64-apple-darwin
	x86_64-apple-darwin24.5-strip target/aarch64-apple-darwin/release/warp-runner || true
	rcodesign sign target/aarch64-apple-darwin/release/warp-runner

	# Windows x86_64
	cargo build -p warp-runner --release --target x86_64-pc-windows-gnu
	strip target/x86_64-pc-windows-gnu/release/warp-runner.exe

	# Windows ARM64 (via zigbuild)
	cargo zigbuild -p warp-runner --release --target aarch64-pc-windows-gnullvm
	# No strip here - MSVC generates PDBs and stripping is not needed

	# --- Build warp-packer after all warp-runner builds are complete ---

	# Linux x86_64
	cargo zigbuild -p warp-packer --release --target x86_64-unknown-linux-gnu.2.28
	strip target/x86_64-unknown-linux-gnu/release/warp-packer

	# Linux aarch64
	cargo zigbuild -p warp-packer --release --target aarch64-unknown-linux-gnu.2.28
	aarch64-linux-gnu-strip target/aarch64-unknown-linux-gnu/release/warp-packer

	# macOS x86_64
	cargo build -p warp-packer --release --target x86_64-apple-darwin
	x86_64-apple-darwin24.5-strip target/x86_64-apple-darwin/release/warp-packer
	rcodesign sign target/x86_64-apple-darwin/release/warp-packer

	# macOS ARM64
	cargo build -p warp-packer --release --target aarch64-apple-darwin
	x86_64-apple-darwin24.5-strip target/aarch64-apple-darwin/release/warp-packer || true
	rcodesign sign target/aarch64-apple-darwin/release/warp-packer

	# Windows x86_64
	cargo build -p warp-packer --release --target x86_64-pc-windows-gnu
	strip target/x86_64-pc-windows-gnu/release/warp-packer.exe

	# Windows ARM64 (via zigbuild)
	cargo zigbuild -p warp-packer --release --target aarch64-pc-windows-gnullvm
	# No strip here - MSVC generates PDBs and stripping is not needed

clean:
	cargo clean

check:
	$(MAKE) build
	$(MAKE) test

test:
	cargo test

.PHONY: all build clean check test
