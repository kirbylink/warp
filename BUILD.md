# Build Instructions

Instructions for building this repository on Linux.

## Table of Contents

* [Build Instructions](#build-instructions)
  * [Table of Contents](#table-of-contents)
  * [Contributing to the Repository](#contributing-to-the-repository)
  * [Repository Content](#repository-content)
  * [Building on Linux](#building-on-linux)
    * [Linux Development Environment Requirements](#linux-development-environment-requirements)
      * [Required Package List](#required-package-list)
      * [Install and Prepare Required Rust Version](#install-and-prepare-required-rust-version)
      * [Install Zig - the compiler engine](#install-zig-the-compiler-engine)
      * [Build the Project](#build-the-project)
      * [Full Build Automation Script (Optional)](#full-build-automation-script-optional)
  * [Building using GitHub Actions](#building-using-github-actions)

## Contributing to the Repository

If you intend to contribute, the preferred workflow is for you to develop your contribution in a fork of this repository in your GitHub account and then submit a pull request.

## Repository Content

This repository contains the source code necessary to build warp-packer for different platforms and architectures.

## Building on Linux

### Linux Development Environment Requirements

This repository has been built and tested on Debian 13.3 (Trixie) on an AMD64 architecture. You should plan for at least **15 GB** of free disk space for all dependencies and build artifacts.

#### Required Package List

```bash
apt install curl git maven
```

#### Install and Prepare Required Rust Version

This repository has been built and tested with Rust version 1.93.1.

To install the newest version, run the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If a newer version doesn't work, a specific version can be installed with the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain=1.93.1
```

For more information see this [GitHub Issue](https://github.com/rust-lang/rustup/issues/2882) and the [official installation instructions](https://rust-lang.github.io/rustup/installation/other.html).

After installing, run the following commands to get the necessary targets:

```bash
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-gnu
rustup target add aarch64-pc-windows-gnullvm
```

#### Install Zig - the compiler engine

Download and unpack the Zig compiler (tested with version 0.15.2):

```bash
mkdir -p ~/.local/zig
cd ~/.local/zig
curl -LO https://ziglang.org/download/0.15.2/zig-x86_64-linux-0.15.2.tar.xz
tar -xf zig-x86_64-linux-0.15.2.tar.xz
rm zig-x86_64-linux-0.15.2.tar.xz
export PATH="$PATH:$HOME/.local/zig"
```

#### Build the Project

Clone the project:

```bash
git clone https://github.com/kirbylink/warp.git
cd warp
```

And within the project directory, run:

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

#### Full Build Automation Script (Optional)

If you'd like to automate the full setup from downloading dependencies to compiling `warp`, you can use the following build script.

1. Install system dependencies as described in the [Required Package List](#required-package-list) (with `apt` and root access).
2. Download the script [build.sh](build.sh) and make it executable:

```bash
chmod +x build.sh
```

3. Run it with an optional directory argument (e.g. on a mounted drive):

```bash
./build.sh /mnt/build-folder
```

If no argument is passed, the script uses `$HOME`.

This script installs:

* Rust with `rustup` and the correct toolchain
* Required Rust targets (see above)
* `cargo-zigbuild`
* Zigland
* warp repository and compiles it with `make`

> You can modify the script to suit your environment. It is designed to work with user privileges (except for `apt`).
>
> **Note:** You should only run the script once to set everything up. Afterward, you only need to run `make` in the cloned warp repository to rebuild it.

## Building using GitHub Actions

If you prefer not to install all platform-specific dependencies locally, this repository provides a GitHub Actions workflow to automatically build all `warp-runner` and `warp-packer` binaries for major platforms and architectures.

**Workflow file**: [`build.yml`](.github/workflows/build.yml)  
**GitHub Actions Documentation**: [Using workflows in your repository](https://docs.github.com/en/actions/using-workflows)

### How to use

You can trigger a full build of all binaries using GitHub Actions - **without creating a release**:

1. Fork the project
2. Open the **["Actions" tab](../../actions)** of your repository on GitHub.
3. Select the **"Build warp-runner and warp-packer"** workflow.
4. Click the **"Run workflow"** button and choose the desired branch.
5. Wait for the workflow to complete.

Once finished, you can download all generated binaries from the **Artifacts** section at the bottom of the workflow run.

### Available binaries (naming pattern)

Each platform-specific binary will be named as follows:

* `linux-x64.warp-packer`
* `linux-aarch64.warp-packer`
* `windows-x64.warp-packer.exe`
* `windows-aarch64.warp-packer.exe`
* `macos-x64.warp-packer`
* `macos-aarch64.warp-packer`

All files are available as artifacts for **90 days**.

> **Note:**
>
> * The build uses GitHub-hosted runners including `ubuntu-24.04-arm` and `windows-11-arm`, which are currently in public preview. See [here](https://github.blog/changelog/2025-01-16-linux-arm64-hosted-runners-now-available-for-free-in-public-repositories-public-preview/) and [here](https://github.blog/changelog/2025-04-14-windows-arm64-hosted-runners-now-available-in-public-preview/).
> * No release is required - the binaries are available directly from the Actions UI.
> * The Windows binaries built using GitHub-hosted runners use the **MSVC toolchain** and therefore **require the [Visual C++ Redistributable for x64](https://aka.ms/vs/17/release/vc_redist.x64.exe)** to be installed on the target machine.  
>   If you're missing `VCRUNTIME140.dll`, installing this runtime should resolve the issue.
> * Alternatively, you can use the cross-compiled Windows binaries built under Linux (with the GNU toolchain), which do **not** require additional runtimes.
