# Build Instructions

Instructions for building this repository on Linux.

## Table Of Contents

- [Build Instructions](#build-instructions)
  - [Table Of Contents](#table-of-contents)
  - [Contributing to the Repository](#contributing-to-the-repository)
  - [Building On Linux](#building-on-linux)
    - [Linux Development Environment Requirements](#linux-development-environment-requirements)
      - [Required Package List](#required-package-list)
      - [Install and prepare required Rust version](#install-and-prepare-required-rust-version)
      - [Required MacOSX Sdk](required-macosx-sdk)
        - [Build and use MacOS Sdk from XCode](build-and-use-macos-sdk-from-xcode)
      - [Build the Project](#build-the-project)

## Contributing to the Repository

If you intend to contribute, the preferred work flow is for you to develop
your contribution in a fork of this repository in your GitHub account and then
submit a pull request.

## Repository Content

This repository contains the source code necessary to build warp-packer for different platforms and architecture.

## Building On Linux

### Linux Development Environment Requirements

This repository has been built and tested on Debian 12.5 (Bookworm) on an AMD64 architecture.

#### Required Package List

`apt install curl maven clang cmake libssl-dev zlib1g-dev liblzma-dev libbz2-dev gcc-aarch64-linux-gnu gcc-mingw-w64-x86-64-win32`

#### Install and prepare required Rust version

This repository has been built and tested with rust version 1.78.

To install the newest version, run the following command:
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

If newer version won't work, a specific version can be used with the following command:
`$ curl --proto '=https' --tls1.2 -sSf https://sh.rustup.rs/ | sh -s --default-toolchain=1.78.0`

For more information see this [Github Issue](https://github.com/rust-lang/rustup/issues/2882).

After install run the following commands to get the necessary targets:
`rustup target add x86_64-apple-darwin`
`rustup target add aarch64-unknown-linux-gnu`
`rustup target add x86_64-pc-windows-gnu`

#### Required MacOSX Sdk

To build the warp-packer for the target x86_64-apple-darwin a MacOSX Sdk is needed. The repository has been build and tested with MacOSX10.12 Sdk.
There are seveal Github repository available that contains different Sdk versions but they all seems to miss the header files. So it is recommend to download it from apples websites.
The MacOS Sdk is integrated in XCode and this repository has been build and tested with Xcode version 8.3.3.

##### Build and use MacOS Sdk from XCode

To build and use the MacOS Sdk from XCode [osxcross](https://github.com/tpoechtrager/osxcross) will be used.

Download osxcross:
`git clone https://github.com/tpoechtrager/osxcross.git`

Extract MacOS Sdk from XCode (see [here](https://github.com/tpoechtrager/osxcross?tab=readme-ov-file#packing-the-sdk-on-linux---method-1-xcode--80):
`<path/to>/osxcross/tools/gen_sdk_package_pbzx.sh <path/to>/Xcode_8.3.3.xip`
Hint: This method may require up to 45 GB of free disk space.
An SSD is recommended for this method.

Copy or move the SDK into the `<path/to>/osxcross/tarballs/` directory.

Run:
`UNATTENDED=yes OSX_VERSION_MIN=10.12 ./build.sh` to create macOS cross toolchain.

Add the target/bin folder to your PATH environment variable:
`PATH="<path/to>/osxcross/target/bin:$PATH"`

#### Build the Project

Clone the project and within the project run
`make`
to start the build.

The compiled warp-packer files are in the folder:
`<path/to/>warp/target/<target>/release/warp-packer`
e.g.
`warp/target/aarch64-unknown-linux-gnu/release/warp-packer`