# Installation Instructions

Instructions for installing the warp-packer executable on Linux, Mac, and Windows.

## Table of Contents

- [Installation Instructions](#installation-instructions)
  - [Table of Contents](#table-of-contents)
  - [Linux](#linux)
  - [Mac](#mac)
    - [Gatekeeper and Quarantine Attribute](#gatekeeper-and-quarantine-attribute)
  - [Windows](#windows)

## Linux

1. **Copy the warp-packer executable to a directory in your PATH:**

   For example, you can copy it to `/usr/local/bin`:
   ```sh
   sudo cp /path/to/warp-packer /usr/local/bin/
   ```

2. **Ensure the warp-packer executable has the correct permissions:**

   Make the file executable:
   ```sh
   sudo chmod +x /usr/local/bin/warp-packer
   ```

3. **Verify the installation:**

   Check that the executable is in your PATH and works correctly:
   ```sh
   warp-packer --version
   ```

## Mac

1. **Copy the warp-packer executable to a directory in your PATH:**

   For example, you can copy it to `/usr/local/bin`:
   ```sh
   sudo cp /path/to/warp-packer /usr/local/bin/
   ```

2. **Ensure the warp-packer executable has the correct permissions:**

   Make the file executable:
   ```sh
   sudo chmod +x /usr/local/bin/warp-packer
   ```

3. **Verify the installation:**

   Check that the executable is in your PATH and works correctly:
   ```sh
   warp-packer --version
   ```

### Gatekeeper and Quarantine Attribute

When downloading executables from the internet, macOS may mark them with a **quarantine attribute**.  
This security mechanism is part of Apple's *Gatekeeper* and can prevent unsigned binaries from running.

If macOS reports that the file is damaged or refuses to execute it, you can remove the quarantine attribute with:

```sh
xattr -d com.apple.quarantine /usr/local/bin/warp-packer
````

After removing the attribute, try running the program again:

```sh
warp-packer --version
```

This step may be necessary when installing binaries that are not signed with an Apple Developer certificate.

## Windows

1. **Copy the warp-packer executable to a directory in your PATH:**

   For example, you can copy it to `C:\Program Files\warp-packer`:
   ```powershell
   Copy-Item -Path "C:\path\to\warp-packer.exe" -Destination "C:\Program Files\warp-packer\"
   ```

2. **Add the directory to your PATH environment variable:**

   - Open the Start Search, type in "env", and select "Edit the system environment variables".
   - In the System Properties window, click on the "Environment Variables..." button.
   - In the Environment Variables window, under "System variables", find the `Path` variable, select it, and click "Edit...".
   - In the Edit Environment Variable window, click "New" and add the path to the directory where you copied the warp-packer executable (`C:\Program Files\warp-packer\`).
   - Click "OK" to close all windows.

3. **Verify the installation:**

   Open a new Command Prompt and check that the executable is in your PATH and works correctly:
   ```cmd
   warp-packer --version
   ```

By following these steps, you will have the warp-packer executable installed and ready to use on Linux, Mac, and Windows.