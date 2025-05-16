# Build Instructions

Instructions for building this repository on Linux.

## Table of Contents

- [Build Instructions](#build-instructions)
  - [Table of Contents](#table-of-contents)
  - [Contributing to the Repository](#contributing-to-the-repository)
  - [Repository Content](#repository-content)
  - [Building on Linux](#building-on-linux)
    - [Linux Development Environment Requirements](#linux-development-environment-requirements)
      - [Required Package List](#required-package-list)
      - [Install and Prepare Required Rust Version](#install-and-prepare-required-rust-version)
      - [Required macOS SDK](#required-macos-sdk)
        - [Build and Use macOS SDK from Xcode](#build-and-use-macos-sdk-from-xcode)
      - [Build the Project](#build-the-project)

## Contributing to the Repository

If you intend to contribute, the preferred workflow is for you to develop your contribution in a fork of this repository in your GitHub account and then submit a pull request.

## Repository Content

This repository contains the source code necessary to build warp-packer for different platforms and architectures.

## Building on Linux

### Linux Development Environment Requirements

This repository has been built and tested on Debian 12.10 (Bookworm) on an AMD64 architecture.

#### Required Package List

```bash
apt install curl maven clang cmake libssl-dev zlib1g-dev liblzma-dev libbz2-dev gcc-aarch64-linux-gnu gcc-mingw-w64-x86-64-win32 git
```

#### Install and Prepare Required Rust Version

This repository has been built and tested with Rust version 1.86.

To install the newest version, run the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If a newer version doesn't work, a specific version can be installed with the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain=1.86.0
```

For more information see this [GitHub Issue](https://github.com/rust-lang/rustup/issues/2882) and the [official installation instructions](https://rust-lang.github.io/rustup/installation/other.html).

After installing, run the following commands to get the necessary targets:

```bash
rustup target add x86_64-apple-darwin
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
```

> Note: Additional targets like `aarch64-apple-darwin` and `aarch64-pc-windows-gnu` can be added later if ARM64 support for macOS and Windows is required.

#### Required macOS SDK

To build warp-packer for the target `x86_64-apple-darwin`, a macOS SDK is needed. The repository has been built and tested with `MacOSX11.3.sdk`.

There are several GitHub repositories available that contain different SDK versions, but they all seem to miss the header files. So it is recommended to download it from Apple's website.

The macOS SDK is integrated into Xcode and Command Line Tools for Xcode. This repository has been built and tested with Command Line Tools for Xcode version 12.5.1 (`Command_Line_Tools_for_Xcode_12.5.1.dmg`).

##### Build and Use macOS SDK from Command Line Tools for Xcode

To build and use the macOS SDK from Xcode, [osxcross](https://github.com/tpoechtrager/osxcross) will be used.

Download osxcross:

```bash
git clone https://github.com/tpoechtrager/osxcross.git
```

Extract the macOS SDK from Command Line Tools for Xcode:

```bash
<path/to>/osxcross/tools/gen_sdk_package_tools_dmg.sh <path/to>/Command_Line_Tools_for_Xcode_12.5.1.dmg
```

Copy or move only the SDK `MacOSX11.3.sdk` from `<path/to>/osxcross/`into the `<path/to>/osxcross/tarballs/` directory.
You may remove older or conflicting SDKs like `10.15` or `11` to avoid issues.

Run the build script to create the macOS cross toolchain:

```bash
UNATTENDED=yes OSX_VERSION_MIN=11 SDK_VERSION=11.3 <path/to>/osxcross/build.sh
```

After that, add the `target/bin` folder to your `PATH` environment variable:

```bash
PATH="<path/to>/osxcross/target/bin:$PATH"
```

#### Build the Project

Clone the project and within the project directory, run:

```bash
make
```

to start the build.

The compiled warp-packer files are in the folder:

```bash
<path/to>/warp/target/<target>/release/warp-packer
```

e.g.

```bash
<path/to>/warp/target/aarch64-unknown-linux-gnu/release/warp-packer
```
