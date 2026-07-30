![Logo](./Assets/Square150x150Logo.png)

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

## Technical highlights
- **Self-contained:** The audio file is baked into the binary.
- **Zero runtime dependencies:** The app interfaces with the Win32 API (via the `windows` crate)
to play audio and provide context menu support.
- **Minimal footprint:** The executable is only ~100KiB (audio included), and it doesn't run in the background.
- **Privacy-first:** No artificial intelligence, only avian intelligence.

## Structure
- `quack.exe`: The main entry point handling the `quack://` protocol and the Copilot key.
- `quack_shell.dll`: The COM shell extension that provides the context menu integration.

## Requirements
- **Windows 11**

## How to build & install
**Prerequisites:** [Rust](https://www.rust-lang.org/tools/install), [winapp](https://github.com/microsoft/winappCli)
1. **Build the project:**
    ```pwsh
    cargo build --release
    ```
2. **Prepare packaging directory:**
    ```pwsh
    mkdir msix
    cp ./target/release/quack.exe ./msix/
    cp ./target/release/quack_shell.dll ./msix/
    ```
3. **Generate a certificate** (first time only):
    ```pwsh
    winapp cert generate --if-exists skip
    ```
4. **Install the certificate** (first time only, requires admin rights):
    ```pwsh
    winapp cert install ./devcert.pfx
    ```
5. **Package the app:**
    ```pwsh
    winapp pack ./msix --cert ./devcert.pfx
    ```
6. **Install the generated `.msix` package.**
7. **Set Quack as your Copilot key provider**
    - Go to **Settings > Bluetooth & devices > Keyboard > Customize Copilot key on keyboard**.
    - Select **Quack**.
8. **Have fun!**

## Minimal build
If you only want to build the Copilot key provider without the extra features:
1. **Build the project:**
    ```pwsh
    cargo build --bin quack --release
    ```
2. **Prepare packaging directory:**
    ```pwsh
    mkdir msix
    cp ./target/release/quack.exe ./msix/
    ```
3. **Generate and install a certificate if necessary** (see above).
4. **Package the app:**
    ```pwsh
    winapp pack ./msix --cert ./devcert.pfx --manifest ./AppxManifest.minimal.xml
    ```
5. **Install and set up the app** (see above).
6. **Have (slightly less) fun!**

## FAQ
> **Q:** Is this a joke?  
> **A:** Less of a joke than a dedicated hardware key for a chatbot.

> **Q:** Is Linux support planned?  
> **A:** No. You could use [keyd](https://github.com/rvaiya/keyd) to remap the Copilot key to something useful instead.

> **Q:** You said it was about Copilot - why all the extra features?  
> **A:** I got carried away.

## License
Quack is licensed under the [MIT License](LICENSE.md).