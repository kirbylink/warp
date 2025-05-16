# Warp

Warp lets you create self-contained single binary applications, making it easier to deliver your software to end users without requiring them to install runtimes or dependencies.

Warp is written in Rust and supports **Linux**, **Windows**, and **macOS**.

---

## Table of Contents

- [Changes in v1.0.0](#changes-in-v100)
- [Quickstart with Node.js](#quickstart-with-nodejs)
  - [Linux](#linux)
  - [macOS](#macos)
  - [Windows](#windows)
- [Quickstart with .NET Core](#quickstart-with-net-core)
  - [Linux](#linux-1)
  - [macOS](#macos-1)
  - [Windows](#windows-1)
- [Quickstart with Java](#quickstart-with-java)
- [How it Works](#how-it-works)
- [Authors](#authors)
- [License](#license)

---

## Changes in v1.0.0

This fork introduces several improvements and compatibility fixes compared to the original [Warp repository](https://github.com/dgiagio/warp). For a full comparison, see the [diff](https://github.com/dgiagio/warp/compare/master...kirbylink:warp:master).

### Breaking Changes

The default invocation has changed from:

```bash
warp-packer <args>
````

to:

```bash
warp-packer pack <args>
```

### New CLI Options

* `-i, --input-dir`: Directory with your application and dependencies
* `-q, --unique-id`: Creates a unique ID for each package (for testing/multiple versions)
* `-p, --prefix`: Set a custom name for the extraction directory
* `-n, --no-clean`: Prevents overwriting extracted versions in the cache

---

## Quickstart with Node.js
### Linux
**Create the directory for the application**
```sh
mkdir myapp
cd myapp
```

**Create main application** - `app.js`
```javascript
var lodash = require('lodash');
var output = lodash.without([1, 2, 3], 1);
console.log(output);
```

**Download Node.js distribution**
```sh
wget https://nodejs.org/dist/v8.12.0/node-v8.12.0-linux-x64.tar.xz
xz -dc node-v8.12.0-linux-x64.tar.xz | tar xvf -
```

**Install dependencies**
```sh
node-v8.12.0-linux-x64/bin/npm install lodash
```

**Remove unneeded files**
```sh
rm -r node-v8.12.0-linux-x64/include node-v8.12.0-linux-x64/share node-v8.12.0-linux-x64/lib
rm node-v8.12.0-linux-x64/bin/npm node-v8.12.0-linux-x64/bin/npx
```

**Create launcher script** - `launch`
```sh
#!/bin/sh

NODE_DIST=node-v8.12.0-linux-x64
APP_MAIN_JS=app.js

DIR="$(cd "$(dirname "$0")" ; pwd -P)"
NODE_EXE=$DIR/$NODE_DIST/bin/node
NODE_PATH=$DIR/node_modules
APP_MAIN_JS_PATH=$DIR/$APP_MAIN_JS

exec $NODE_EXE $APP_MAIN_JS_PATH $@
```

**Make the launcher script executable**
```sh
chmod +x launch
```

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```sh
cd ..
curl -Lo warp-packer https://github.com/kirbylink/warp/releases/download/1.1.0/linux-x64.warp-packer
chmod +x warp-packer
```

**Create your self-contained application**

```sh
./warp-packer pack --arch linux-x64 --input-dir myapp --exec launch --output myapp.bin
chmod +x myapp.bin
```

**Run your self-contained application**

```sh
./myapp.bin
[ 2, 3 ]
```

**More information about your self-contained application**

```sh
file myapp.bin
myapp.bin: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 3.2.0, BuildID[sha1]=aa53b01be2cde5e0b64450870b1af13b52d5cffb, with debug_info, not stripped

du -hs myapp.bin
17M     myapp.bin
```

### macOS
**Create the directory for the application**
```sh
mkdir myapp
cd myapp
```

**Create main application** - `app.js`
```javascript
var lodash = require('lodash');
var output = lodash.without([1, 2, 3], 1);
console.log(output);
```

**Download Node.js distribution**
```sh
curl -Lo node-v8.12.0-darwin-x64.tar.gz https://nodejs.org/dist/v8.12.0/node-v8.12.0-darwin-x64.tar.gz
tar xvfz node-v8.12.0-darwin-x64.tar.gz
```

**Install dependencies**
```sh
PATH=node-v8.12.0-darwin-x64/bin npm install lodash
```

**Remove unneeded files**
```sh
rm -r node-v8.12.0-darwin-x64/include node-v8.12.0-darwin-x64/share node-v8.12.0-darwin-x64/lib
rm node-v8.12.0-darwin-x64/bin/npm node-v8.12.0-darwin-x64/bin/npx
```

**Create launcher script*** - `launch`
```sh
#!/bin/sh

NODE_DIST=node-v8.12.0-darwin-x64
APP_MAIN_JS=app.js

DIR="$(cd "$(dirname "$0")" ; pwd -P)"
NODE_EXE=$DIR/$NODE_DIST/bin/node
NODE_PATH=$DIR/node_modules
APP_MAIN_JS_PATH=$DIR/$APP_MAIN_JS

exec "$NODE_EXE" "$APP_MAIN_JS_PATH" $@
```

**Make the launcher script executable**
```sh
chmod +x launch
```

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```sh
cd ..
curl -Lo warp-packer https://github.com/kirbylink/warp/releases/download/1.1.0/macos-x64.warp-packer
chmod +x warp-packer
```

**Create your self-contained application**

```sh
./warp-packer pack --arch macos-x64 --input-dir myapp --exec launch --output myapp.bin
chmod +x myapp.bin
```

**Run your self-contained application**

```sh
./myapp.bin
[ 2, 3 ]
```

**More information about your self-contained application**

```sh
file myapp.bin
myapp.bin: Mach-O 64-bit executable x86_64

du -hs myapp.bin
26M     myapp.bin
```

### Windows
**Create the directory for the application**
```powershell
mkdir myapp
cd myapp
```

**Create main application** - `app.js`
```javascript
var lodash = require('lodash');
var output = lodash.without([1, 2, 3], 1);
console.log(output);
```

**Download Node.js distribution**
```powershell
curl https://nodejs.org/dist/v8.12.0/node-v8.12.0-win-x64.zip -OutFile node-v8.12.0-win-x64.zip
Expand-Archive .\node-v8.12.0-win-x64.zip -DestinationPath .\
```

**Install dependencies**
```powershell
.\node-v8.12.0-win-x64\npm install lodash
```

**Remove unneeded files**
```powershell
rmdir -Recurse .\node-v8.12.0-win-x64\node_modules\npm
```

**Create launcher script*** - `launch.cmd`
```bat
@ECHO OFF

SETLOCAL

SET "NODE_DIST=node-v8.12.0-win-x64"
SET "APP_MAIN_JS=app.js"

SET "NODE_EXE=%~dp0\%NODE_DIST%\node.exe"
SET "NODE_PATH=%~dp0\%NODE_DIST%\node_modules"
SET "APP_MAIN_JS_PATH=%~dp0\%APP_MAIN_JS%"

CALL %NODE_EXE% %APP_MAIN_JS_PATH% %*
EXIT /B %ERRORLEVEL%
```

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```powershell
cd ..
[Net.ServicePointManager]::SecurityProtocol = "tls12, tls11, tls" ; Invoke-WebRequest https://github.com/kirbylink/warp/releases/download/1.1.0/windows-x64.warp-packer.exe -OutFile warp-packer.exe
```

**Create your self-contained application**

```powershell
.\warp-packer --arch windows-x64 --input-dir .\myapp\ --exec launch.cmd --output myapp.exe
```

**Run your self-contained application**

```powershell
.\myapp.exe
[ 2, 3 ]
PS $HOME>
```

**More information about your self-contained application**

```powershell
"{0:N2} MB" -f ((Get-Item myapp.exe).Length / 1MB)
9.15 MB
```

---

## Quickstart with .NET Core
### Linux
**Create a simple console application**

```sh
mkdir myapp
cd myapp
dotnet new console
dotnet run
Hello World!
```

**Publish the application with native installer for `linux-x64` runtime**

```sh
dotnet publish -c Release -r linux-x64
```
The application should be published to `bin/Release/netcoreapp2.1/linux-x64/publish/`

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```sh
curl -Lo warp-packer https://github.com/kirbylink/warp/releases/download/1.1.0/linux-x64.warp-packer
chmod +x warp-packer
```

**Create your self-contained application**

```sh
./warp-packer pack --arch linux-x64 --input-dir bin/Release/netcoreapp2.1/linux-x64/publish --exec myapp --output myapp
chmod +x myapp
```

**Run your self-contained application**

```sh
./myapp
Hello World!
```

**More information about your self-contained application**

```sh
file myapp
myapp: ELF 64-bit LSB executable, x86-64, version 1 (GNU/Linux), statically linked, BuildID[sha1]=13b12e71a63ca1de8537ad7e90c83241f9f87f6c, with debug_info, not stripped

du -hs myapp
34M     myapp
```

### macOS
**Create a simple console application**

```sh
mkdir myapp
cd myapp
dotnet new console
dotnet run
Hello World!
```

**Publish the application with native installer for `osx-x64` runtime**

```sh
dotnet publish -c Release -r osx-x64
```
The application should be published to `bin/Release/netcoreapp2.1/osx-x64/publish/`

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```sh
curl -Lo warp-packer https://github.com/kirbylink/warp/releases/download/1.1.0/macos-x64.warp-packer
chmod +x warp-packer
```

**Create your self-contained application**

```sh
./warp-packer pack --arch macos-x64 --input-dir bin/Release/netcoreapp2.1/osx-x64/publish --exec myapp --output myapp
chmod +x myapp
```

**Run your self-contained application**

```sh
./myapp
Hello World!
```

**More information about your self-contained application**

```sh
file myapp
myapp: Mach-O 64-bit executable x86_64

du -hs myapp
 27M    myapp
```

### Windows
**Create a simple console application**

```powershell
mkdir myapp
cd myapp
dotnet new console
dotnet run
Hello World!
PS $HOME\myapp>
```

**Publish the application with native installer for `win10-x64` runtime**

```powershell
dotnet publish -c Release -r win10-x64
```
The application should be published to `bin/Release/netcoreapp2.1/win10-x64/publish/`

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```powershell
[Net.ServicePointManager]::SecurityProtocol = "tls12, tls11, tls" ; Invoke-WebRequest https://github.com/kirbylink/warp/releases/download/1.1.0/windows-x64.warp-packer.exe -OutFile warp-packer.exe
```

**Create your self-contained application**

```powershell
.\warp-packer --arch windows-x64 --input-dir bin/Release/netcoreapp2.1/win10-x64/publish --exec myapp.exe --output myapp.exe
```

**Run your self-contained application**

```powershell
.\myapp.exe
Hello World!
PS $HOME\myapp>
```

**More information about your self-contained application**

```powershell
"{0:N2} MB" -f ((Get-Item myapp.exe).Length / 1MB)
28.51 MB
```

---

## Quickstart with Java

Warp can package Java JARs along with a minimal JRE and startup scripts.

> 🧠 **Tip:** If you want to skip the manual steps below, check out
> [`java-warp4j`](https://github.com/kirbylink/java-warp4j) — a Java-based tool that automates all of this:
> It handles JDK downloads, `jlink`, script creation, and even runs `warp-packer` internally.

### Manual Instructions

1. **Create a Hello World app**:

```java
// HelloWorld.java
public final class HelloWorld {
  public static void main(final String[] args) {
    System.out.println("Hello, world.");
  }
}
```

2. **Compile and bundle as JAR**:

```bash
javac HelloWorld.java
jar cvfe app.jar HelloWorld HelloWorld.class
```

3. **Download and unpack a JRE** (e.g., from [Adoptium](https://adoptium.net)):

```bash
wget https://.../OpenJDK8U-jre_x64_linux_hotspot_8u412b08.tar.gz
tar -xvf OpenJDK8U-jre_x64_linux_hotspot_8u412b08.tar.gz
```

4. **Create a bundle folder**:

```bash
mkdir bundle
cp -r jdk-folder bundle/jre
cp app.jar bundle/
```

5. **Add a launcher script** (`bundle/run.sh`):

```bash
#!/bin/bash
DIR="$(cd "$(dirname "$0")"; pwd)"
exec "$DIR/jre/bin/java" -jar "$DIR/app.jar" "$@"
```

6. **Make script executable**:

```bash
chmod +x bundle/run.sh
```

7. **Download warp-packer** (if not already in PATH):

```bash
curl -Lo warp-packer https://github.com/kirbylink/warp/releases/download/1.1.0/linux-x64.warp-packer
chmod +x warp-packer
```

8. **Create your binary**:

```bash
./warp-packer pack --arch linux-x64 --input-dir bundle --exec run.sh --output app.bin
chmod +x app.bin
```

9. **Run your binary**:

```bash
./app.bin
```

---

## How it Works

Warp combines your runtime and application into a single file using two tools:

* **`warp-runner`**: The runtime stub that extracts and launches your app
* **`warp-packer`**: CLI tool that builds the binary, compresses files, and fetches `warp-runner`

<img src="./documentation/warp_app_binary.png" width="272">

### First-time Run

The first execution unpacks your app to a cache directory and runs it. Subsequent executions run directly.

### Cache Directories

* Linux: `$HOME/.local/share/warp/`
* macOS: `$HOME/Library/Application Support/warp/`
* Windows: `%LOCALAPPDATA%\warp\`

---

## Authors

* Original Author: Diego Giagio `<diego@giagio.com>`
* Modifications and enhancements by: [@kirbylink](https://github.com/kirbylink)

---

## License

This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.

---

## Who Uses Warp?

* [Buckaroo](https://github.com/loopperfect/buckaroo): C++ package manager
* [Buck (Warp)](https://github.com/njlr/buck-warp): Build system wrapper