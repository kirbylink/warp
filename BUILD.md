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
      * [Required macOS SDK](#required-macos-sdk)
        * [Build and Use macOS SDK from Xcode](#build-and-use-macos-sdk-from-xcode)
        * [Prepare Cross-Compilation for Windows ARM64](#prepare-cross-compilation-for-windows-arm64)
      * [Build the Project](#build-the-project)
      * [Full Build Automation Script (Optional)](#full-build-automation-script-optional)
  * [Building using GitHub Actions](#building-using-github-actions)

## Contributing to the Repository

If you intend to contribute, the preferred workflow is for you to develop your contribution in a fork of this repository in your GitHub account and then submit a pull request.

## Repository Content

This repository contains the source code necessary to build warp-packer for different platforms and architectures.

## Building on Linux

### Linux Development Environment Requirements

This repository has been built and tested on Debian 12.10 (Bookworm) on an AMD64 architecture. You should plan for at least **15 GB** of free disk space for all dependencies and build artifacts.

#### Required Package List

```bash
apt install curl maven clang cmake libssl-dev zlib1g-dev liblzma-dev libbz2-dev gcc-aarch64-linux-gnu gcc-mingw-w64-x86-64-win32 git llvm lld
```

#### Install and Prepare Required Rust Version

This repository has been built and tested with Rust version 1.87.

To install the newest version, run the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If a newer version doesn't work, a specific version can be installed with the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain=1.87.0
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

#### Required macOS SDK

To build warp-packer for the target `x86_64-apple-darwin`, a macOS SDK is needed. The repository has been built and tested with `MacOSX11.3.sdk`.

There are several GitHub repositories available that contain different SDK versions, but they all seem to miss the header files. So it is recommended to download it from Apple's website.

The macOS SDK is integrated into Xcode and Command Line Tools for Xcode. This repository has been built and tested with Command Line Tools for Xcode version 12.5.1 (`Command_Line_Tools_for_Xcode_12.5.1.dmg`).

A free Apple account is needed to download the Command Line tools.

1. Go to: [https://account.apple.com/sign-in](https://account.apple.com/sign-in) and log in.
2. Then visit: [https://developer.apple.com/download/more/](https://developer.apple.com/download/more/)
3. Search for: `Command_Line_Tools_for_Xcode_12.5.1.dmg`

The direct download link may change, but as of this writing it is:

```
https://download.developer.apple.com/Developer_Tools/Command_Line_Tools_for_Xcode_12.5.1/Command_Line_Tools_for_Xcode_12.5.1.dmg
```

The resulting Clang and AR tools (e.g. `aarch64-apple-darwin20.4-clang` and `aarch64-apple-darwin20.4-ar`) must be referenced in the `.cargo/config.toml` file accordingly.

> ⚠️ **Important:** The version `Command Line Tools for Xcode 12.5.1` must be used exactly as described. Using a different version will change the internal SDK version (e.g. `MacOSX11.3.sdk`) and toolchain prefix (e.g. `aarch64-apple-darwin20.4-clang`), which in turn requires manual adjustments to both the `Makefile` and `.cargo/config.toml`. This setup has been tested and validated only with version 12.5.1.

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

Copy or move only the SDK `MacOSX11.3.sdk` from `<path/to>/osxcross/` into the `<path/to>/osxcross/tarballs/` directory.
You may remove older or conflicting SDKs like `10.15` or `11` to avoid issues.

Run the build script to create the macOS cross toolchain:

```bash
UNATTENDED=yes OSX_VERSION_MIN=11 SDK_VERSION=11.3 <path/to>/osxcross/build.sh
```

After that, add the `target/bin` folder to your `PATH` environment variable:

```bash
export PATH="<path/to>/osxcross/target/bin:$PATH"
```

To cross-compile with the osxcross toolchain, make sure your `.cargo/config.toml` includes entries like this (adjust paths if needed):

```toml
[target.x86_64-apple-darwin]
linker = "x86_64-apple-darwin20.4-clang"
ar = "x86_64-apple-darwin20.4-ar"

[target.aarch64-apple-darwin]
linker = "aarch64-apple-darwin20.4-clang"
ar = "aarch64-apple-darwin20.4-ar"
```

These toolchain names depend on the SDK version and osxcross build result. If you use a different SDK, the version suffix (like 20.4) will likely change.

##### Prepare Cross-Compilation for Windows ARM64

To compile for Windows ARM64 (`aarch64-pc-windows-gnullvm`), the [`cargo-zigbuild`](https://github.com/messense/cargo-zigbuild) tool is used. It integrates the Zig compiler to simplify cross-compilation.

Install `cargo-zigbuild`:

```bash
cargo install cargo-zigbuild
```

Download and unpack the Zig compiler (tested with version 0.14.0):

```bash
mkdir -p ~/.local/zig
cd ~/.local/zig
curl -LO https://ziglang.org/download/0.14.0/zig-linux-x86_64-0.14.0.tar.xz
tar -xf zig-linux-x86_64-0.14.0.tar.xz
export PATH="$HOME/.local/zig/zig-linux-x86_64-0.14.0:$PATH"
```

This target uses the LLVM-based ABI and is officially supported by Rust. It is currently the most reliable option for producing ARM64 Windows binaries on Linux.

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
2. Download the script (e.g. `build.sh`) and make it executable:

```bash
chmod +x build.sh
```

3. Run it with an optional directory argument (e.g. on a mounted drive):

```bash
./build.sh /mnt/build-folder
```

If no argument is passed, the script uses `$HOME`.

4. You still need to provide the path to the downloaded `Command_Line_Tools_for_Xcode_12.5.1.dmg` inside the script.

This script installs:

* Rust with `rustup` and the correct toolchain
* Required Rust targets (see above)
* `cargo-zigbuild`
* Zig 0.14.0
* osxcross (with `MacOSX11.3.sdk` extracted and built)
* warp repository and compiles it with `make`

> You can modify the script to suit your environment. It is designed to work with user privileges (except for `apt`).
>
> **Note:** You should only run the script once to set everything up. Afterward, you only need to run `make` in the cloned warp repository to rebuild it. However, the paths to the tools (like `osxcross`, `zig`, or the SDK) still need to be correctly set in your environment (e.g. via `PATH` or `.cargo/config.toml`).

## Building using GitHub Actions

If you prefer not to install all platform-specific dependencies locally, this repository provides a GitHub Actions workflow to automatically build all `warp-runner` and `warp-packer` binaries for major platforms and architectures.

**Workflow file**: [`build.yml`](.github/workflows/build.yml)  
**GitHub Actions Documentation**: [Using workflows in your repository](https://docs.github.com/en/actions/using-workflows)

### How to use

You can trigger a full build of all binaries using GitHub Actions - **without creating a release**:

1. Open the **["Actions" tab](../../actions)** of your repository on GitHub.
2. Select the **"Build warp-runner and warp-packer"** workflow.
3. Click the **"Run workflow"** button and choose the desired branch.
4. Wait for the workflow to complete.

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
