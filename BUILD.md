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

This repository has been built and tested on Debian 12.5 (Bookworm) on an AMD64 architecture.

#### Required Package List

```bash
apt install curl maven clang cmake libssl-dev zlib1g-dev liblzma-dev libbz2-dev gcc-aarch64-linux-gnu gcc-mingw-w64-x86-64-win32
```

#### Install and Prepare Required Rust Version

This repository has been built and tested with Rust version 1.78.

To install the newest version, run the following command:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If a newer version doesn't work, a specific version can be installed with the following command:
```bash
$ curl --proto '=https' --tls1.2 -sSf https://sh.rustup.rs/ | sh -s --default-toolchain=1.78.0
```

For more information see this [GitHub Issue](https://github.com/rust-lang/rustup/issues/2882).

After installing, run the following commands to get the necessary targets:
```bash
rustup target add x86_64-apple-darwin
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
```

#### Required macOS SDK

To build warp-packer for the target x86_64-apple-darwin, a macOS SDK is needed. The repository has been built and tested with MacOSX10.12 SDK.
There are several GitHub repositories available that contain different SDK versions, but they all seem to miss the header files. So it is recommended to download it from Apple's website.
The macOS SDK is integrated into Xcode, and this repository has been built and tested with Xcode version 8.3.3.

##### Build and Use macOS SDK from Xcode

To build and use the macOS SDK from Xcode, [osxcross](https://github.com/tpoechtrager/osxcross) will be used.

Download osxcross:
```bash
git clone https://github.com/tpoechtrager/osxcross.git
```

Extract macOS SDK from Xcode (see [here](https://github.com/tpoechtrager/osxcross?tab=readme-ov-file#packing-the-sdk-on-linux---method-1-xcode--80)):
```bash
<path/to>/osxcross/tools/gen_sdk_package_pbzx.sh <path/to>/Xcode_8.3.3.xip
```

Hint: This method may require up to 45 GB of free disk space. An SSD is recommended for this method.

Copy or move the SDK into the `<path/to>/osxcross/tarballs/` directory.

Run:
```bash
UNATTENDED=yes OSX_VERSION_MIN=10.12 ./build.sh
```
to create the macOS cross toolchain.

Add the target/bin folder to your PATH environment variable:
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
warp/target/aarch64-unknown-linux-gnu/release/warp-packer
```