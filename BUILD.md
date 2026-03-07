# Build Instructions

BUILD.md containing instructions on how to build warp-packer.

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
        * [Build and Use macOS SDK from Command Line Tools for Xcode](#build-and-use-macos-sdk-from-command-line-tools-for-xcode)
        * [Additional Tool for macOS ARM64 Compatibility](#additional-tool-for-macos-arm64-compatibility)
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

This repository has been built and tested on Debian 13.3 (Trixie) on an AMD64 architecture. You should plan for at least **15 GB** of free disk space for all dependencies and build artifacts.

#### Required Package List

```bash
apt install curl git tar xz-utils clang cmake libssl-dev zlib1g-dev liblzma-dev libbz2-dev gcc-aarch64-linux-gnu gcc-mingw-w64-x86-64-win32 llvm lld
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

#### Required macOS SDK

To build warp-packer for the target `x86_64-apple-darwin`, a macOS SDK is needed. The repository has been built and tested with `MacOSX15.5.sdk`.

There are several GitHub repositories available that contain different SDK versions, but they all seem to miss the header files. So it is recommended to download it from Apple's website.

The macOS SDK is integrated into Xcode and Command Line Tools for Xcode. This repository has been built and tested with Command Line Tools for Xcode version 16.4 (`Command_Line_Tools_for_Xcode_16.4.dmg`).

A free Apple account is needed to download the Command Line tools.

1. Go to: [https://developer.apple.com/account/](https://developer.apple.com/account/) and log in.
2. Then visit: [https://developer.apple.com/download/more/](https://developer.apple.com/download/more/)
3. Search for: `Command Line Tools for Xcode 16.4`

The direct download link may change, but as of this writing it is:

```
https://download.developer.apple.com/Developer_Tools/Command_Line_Tools_for_Xcode_16.4/Command_Line_Tools_for_Xcode_16.4.dmg
```

The resulting Clang and AR tools (e.g. `aarch64-apple-darwin24.5-clang` and `aarch64-apple-darwin24.5-ar`) must be referenced in the `.cargo/config.toml` file accordingly.

> ⚠️ **Important:** The version `Command Line Tools for Xcode 16.4` must be used exactly as described. Using a different version will change the internal SDK version (e.g. `MacOSX15.5.sdk`) and toolchain prefix (e.g. `aarch64-apple-darwin24.5-clang`), which in turn requires manual adjustments to both the `Makefile` and `.cargo/config.toml`. This setup has been tested and validated only with version 16.4.

##### Build and Use macOS SDK from Command Line Tools for Xcode

To build and use the macOS SDK from Xcode, [osxcross](https://github.com/tpoechtrager/osxcross) will be used.

Download osxcross:

```bash
git clone https://github.com/tpoechtrager/osxcross.git
```

Extract the macOS SDK from Command Line Tools for Xcode:

```bash
<path/to>/osxcross/tools/gen_sdk_package_tools_dmg.sh <path/to>/Command_Line_Tools_for_Xcode_16.4.dmg
```

Copy or move only the SDK `MacOSX15.5.sdk` from `<path/to>/osxcross/` into the `<path/to>/osxcross/tarballs/` directory.
You may remove older or conflicting SDKs like `14.0`, `14.5` or `15.0` to avoid issues.

Run the build script to create the macOS cross toolchain:

```bash
UNATTENDED=yes OSX_VERSION_MIN=11 SDK_VERSION=15.5 <path/to>/osxcross/build.sh
```

After that, add the `target/bin` folder to your `PATH` environment variable:

```bash
export PATH="<path/to>/osxcross/target/bin:$PATH"
```

To cross-compile with the osxcross toolchain, make sure your `.cargo/config.toml` includes entries like this (adjust paths if needed):

```toml
[target.x86_64-apple-darwin]
linker = "x86_64-apple-darwin24.5-clang"
ar = "x86_64-apple-darwin24.5-ar"

[target.aarch64-apple-darwin]
linker = "aarch64-apple-darwin24.5-clang"
ar = "aarch64-apple-darwin24.5-ar"
```

These toolchain names depend on the SDK version and osxcross build result. If you use a different SDK, the version suffix (like 24.5) will likely change.

##### Additional Tool for macOS ARM64 Compatibility

When building binaries for **macOS on Apple Silicon (ARM64)**, macOS may refuse to run unsigned executables.
In such cases the system can display an error like:

> *“The application is damaged and can't be opened.”*

To avoid this issue, the build process uses the tool **`apple-codesign`**, which provides the `rcodesign` utility.
This tool applies an **ad-hoc code signature** to the generated macOS binaries.

An ad-hoc signature does **not require an Apple Developer certificate**.
It simply adds the minimal signature metadata required by macOS so the binary is accepted by the system loader.

Install the tool using Cargo:

```bash
cargo install apple-codesign
```

During the build process, the `Makefile` automatically signs the generated macOS binaries using `rcodesign`. No additional manual steps are required.

#### Prepare Cross-Compilation for Windows ARM64

To compile for Windows ARM64 (`aarch64-pc-windows-gnullvm`), the [`cargo-zigbuild`](https://github.com/messense/cargo-zigbuild) tool is used. It integrates the Zig compiler to simplify cross-compilation.

Install `cargo-zigbuild`:

```bash
cargo install cargo-zigbuild
```

Download and unpack the Zig compiler (tested with version 0.15.2):

```bash
mkdir -p ~/.local/zig
cd ~/.local/zig
curl -LO https://ziglang.org/download/0.15.2/zig-x86_64-linux-0.15.2.tar.xz
tar -xf zig-x86_64-linux-0.15.2.tar.xz
export PATH="$HOME/.local/zig/zig-x86_64-linux-0.15.2:$PATH"
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

The build script additionally bundles all compiled binaries into:

target/bundle/

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

4. You still need to provide the path to the downloaded `Command_Line_Tools_for_Xcode_16.4.dmg` inside the script.

The script performs the following steps automatically:

* installs Rust via rustup
* installs all required Rust compilation targets
* installs cargo-zigbuild and apple-codesign
* downloads and configures Zig
* builds the macOS cross-compilation toolchain using osxcross
* clones the warp repository
* builds warp for all supported targets
* bundles the resulting warp-packer binaries

> You can modify the script to suit your environment. It is designed to work with user privileges (except for `apt`).
>
> **Note:** You should only run the script once to set everything up. Afterward, you only need to run `make` in the cloned warp repository to rebuild it. However, the paths to the tools (like `osxcross`, `zig`, or the SDK) still need to be correctly set in your environment (e.g. via `PATH` or `.cargo/config.toml`).

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
