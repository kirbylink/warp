# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v1.1.0] - 2025-05-18
### Added
- Native build support for `aarch64-windows` (Windows ARM64)
- Native build support for `aarch64-macos` (Apple Silicon)
- GitHub Actions workflow to build `warp-runner` and `warp-packer` binaries for all major platforms (Linux, macOS, Windows - x64 and ARM64)
- Documentation in `BUILD.md` for triggering builds via GitHub Actions

### Changed
- Updated Rust dependencies to latest compatible versions
- Revised and tested `BUILD.md` instructions using newer SDKs and Rust toolchains
- Improved and documented usage of the `build.sh` automation script for local builds

## [v1.0.0] - 2024-05-18
### Changed
- Changed the default invocation to `warp-packer pack <argument1> <argument2> ...`.

For more details, see the [README](./README.md#changes-in-v100).

### Added
- Support for native execution on aarch64-linux platforms, e.g., Raspberry Pi 4.
- New options for `warp-packer`.
- `BUILD.md` with instructions on how to compile warp-packer.
- `INSTALL.md` with instructions on how to run the application on Linux, macOS, and Windows systems.

## [v0.1.0 - v0.3.0] - 2018-10-30
### Changed
- For changes see origin repository: https://github.com/dgiagio/warp/releases

[unreleased]: https://github.com/kirbylink/warp/compare/master...HEAD
[v1.1.0]: https://github.com/kirbylink/warp/compare/v1.0.0...v1.1.0
[v1.0.0]: https://github.com/kirbylink/warp/compare/v0.3.0...v1.0.0
[v0.1.0 - v0.3.0]: https://github.com/dgiagio/warp/releases