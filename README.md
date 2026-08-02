![Logo](pri/Assets/Square150x150Logo.png)

![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
![Rust](https://img.shields.io/badge/Rust-D34516.svg?logo=rust&logoColor=white)
![Windows](https://img.shields.io/badge/Windows-0078D4.svg?logo=data:image/svg%2bxml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIGZpbGw9IndoaXRlIiByb2xlPSJpbWciIHZpZXdCb3g9IjAgMCAyNCAyNCI+PHRpdGxlPldpbmRvd3MgMTE8L3RpdGxlPjxwYXRoIGQ9Ik0wLDBIMTEuMzc3VjExLjM3MkgwWk0xMi42MjMsMEgyNFYxMS4zNzJIMTIuNjIzWk0wLDEyLjYyM0gxMS4zNzdWMjRIMFptMTIuNjIzLDBIMjRWMjRIMTIuNjIzIi8+PC9zdmc+)
![Made in Ukraine](https://img.shields.io/badge/Made%20in-Ukraine-FFD700.svg?labelColor=0057B7)

# Quack

**Unlock the Copilot key's full potential.**

## Background
> In the first age, in the first battle, when the shadows first lengthened, one stood.
Burned by the embers of AI, his keyboard blistered by the fires of Redmond
and tainted beyond usability, he chose the path of perpetual quack.

Got a new keyboard with the Copilot key? Wish it could do something useful?  
This small utility makes it quack!

## Features
- **Copilot key:** Registers as a Copilot hardware key provider.  
*(Press the Copilot key to quack!)*
- **`quack://`:** Registers a system-wide `quack://` protocol handler.  
*(Try `quack://` in your browser!)*
- **Context menu:** Adds a `Quack` entry to the context menu.  
*(Try right-clicking a file!)*
- **Share menu:** Adds a `Duck` entry to the share menu.  
*(Try sharing a file with the duck!)*

## Technical highlights
- **Self-contained:** The audio file is baked into the binary.
- **Zero runtime dependencies:** The app interfaces with the Win32 API (via the `windows` crate)
to play audio and provide context menu support.
- **Minimal footprint:** The `quack` executable is only ~100KiB (audio included), and it doesn't run in the background.
- **Privacy-first:** No artificial intelligence, only avian intelligence.

## Structure
- `quack.exe`: The executable handling the `quack://` protocol and the Copilot key.
- `quack_shell.dll`: The COM shell extension that provides the context menu integration.
- `duck.exe`: The executable handling the share menu integration.

## Requirements
- **Runtime requirements:**
  - **Windows 10** (10.0.18362 or later)
- **Build requirements:**
  - **[Rust](https://www.rust-lang.org/tools/install)**
  - **Windows App Certification Kit**
    (from the [Windows SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk/))

## How to build & install
1. **Build the project:**
    ```pwsh
    cargo build --release
    ```
2. **Create the packaging directory:**
    ```pwsh
   $arch = "{arch}"
   mkdir ./msix/$arch/
    ```
   where `{arch}` is `x64` or `arm64`.

3. **Create .pri file:**
    ```pwsh
    makepri new /pr . /cf priconfig.xml /of ./msix/$arch/resources.pri /o
    ```
4. **Prepare the packaging directory:**
    ```pwsh
    (cat ./AppxManifest.xml).Replace("neutral", "$arch") | Set-Content ./msix/$arch/AppxManifest.xml
    cp ./target/release/quack.exe ./msix/$arch/
    cp ./target/release/quack_shell.dll ./msix/$arch/
    cp ./target/release/duck.exe ./msix/$arch/
    cp -Recurse -Force ./pri/Assets/ ./msix/$arch/
    ```
5. **Generate a certificate** (first time only):
    ```pwsh
    $thumbprint = (New-SelfSignedCertificate -Type Custom -KeyUsage DigitalSignature -CertStoreLocation "Cert:\CurrentUser\My" -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3", "2.5.29.19={text}") -Subject "CN=Tangonell" -FriendlyName "Quack Dev Certificate").Thumbprint
    $passwd = "{password}"
    $securePasswd = ConvertTo-SecureString -String "$passwd" -Force -AsPlainText
    Export-PfxCertificate -cert "Cert:\CurrentUser\My\$thumbprint" -FilePath "./devcert.pfx" -Password $securePasswd
    Import-PfxCertificate -CertStoreLocation "Cert:\LocalMachine\TrustedPeople" -Password $securePasswd -FilePath ./devcert.pfx
    ```
   where `{password}` is the certificate password.

6. **Package the app:**
    ```pwsh
    makeappx pack /d ./msix/$arch/ /p ./msix/quack_$arch.msix /o
    ```
7. **Sign the app:**
    ```pwsh
    signtool sign /fd SHA256 /a /f ./devcert.pfx /p "$passwd" ./msix/quack_$arch.msix
    ```
8. **Install the generated `.msix` package.**
9. **Set Quack as your Copilot key provider** (Windows 11):
    - Go to **Settings > Bluetooth & devices > Keyboard > Customize Copilot key on keyboard**.
    - Select **Quack**.
10. **Have fun!**

## Minimal build
If you only want to build the Copilot key provider without the extra features:

1. **Build the project:**
    ```pwsh
    cargo build --bin quack --release
    ```
2. **Create the packaging directory** (see above).
3. **Create `.pri` file** (see above).
4. **Prepare the packaging directory:**
    ```pwsh
    (cat ./AppxManifest.minimal.xml).Replace("neutral", "$arch") | Set-Content ./msix/$arch/AppxManifest.xml
    cp ./target/release/quack.exe ./msix/$arch/
    cp -Recurse -Force ./pri/Assets/ ./msix/$arch/
    ```
5. **Generate a certificate** (first time only, see above).
6. **Package, sign, install and set up the app** (see above).
7. **Have (slightly less) fun!**

## FAQ
> **Q:** Is this a joke?  
> **A:** Less of a joke than a dedicated hardware key for a chatbot.

> **Q:** Is Linux support planned?  
> **A:** No. You could use [keyd](https://github.com/rvaiya/keyd) to remap the Copilot key to something useful instead.

> **Q:** You said it was about Copilot - why all the extra features?  
> **A:** I got carried away.

## License
Quack is licensed under the [MIT License](LICENSE.md).