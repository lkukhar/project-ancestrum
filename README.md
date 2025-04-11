# Ancestrum - Family Tree Visualizer

A modern desktop application for visualizing and managing family trees, built with Rust and iced.

## Features

- Add, update, and delete ancestors
- Visualize family relationships
- Save and load family tree data
- Cross-platform support (Windows, macOS, Linux)

## Prerequisites

### Windows
1. Install Rust:
   ```powershell
   winget install Rustlang.Rust.MSVC
   ```
   or download from https://rustup.rs/

2. Install Visual Studio Build Tools:
   - Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
   - During installation, select:
     - "Desktop development with C++"
     - "Windows 10 SDK"
     - "MSVC v143 - VS 2022 C++ x64/x86 build tools"

3. Set up environment variables:
   ```powershell
   $env:Path += ";C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.xx.xxxxx\bin\Hostx64\x64"
   ```
   (Replace 14.xx.xxxxx with your installed version)

### macOS
1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install Xcode Command Line Tools:
   ```bash
   xcode-select --install
   ```

3. Install Homebrew (if not already installed):
   ```bash
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   ```

4. Install system dependencies:
   ```bash
   brew install pkg-config
   ```

### Linux
1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install system dependencies:

   Ubuntu/Debian:
   ```bash
   sudo apt update
   sudo apt install build-essential pkg-config libx11-dev libxcb1-dev libxcb-shm0-dev libxcb-xfixes0-dev libxcb-randr0-dev libxcb-composite0-dev libxcb-xkb-dev libxcb-icccm4-dev libxcb-image0-dev libxcb-keysyms1-dev libxcb-util0-dev libxcb-xinerama0-dev libxcb-xkb-dev libxkbcommon-x11-dev libxcb-cursor-dev
   ```

   Fedora:
   ```bash
   sudo dnf install gcc-c++ pkg-config libX11-devel libxcb-devel libxcb-shm0-devel libxcb-xfixes0-devel libxcb-randr0-devel libxcb-composite0-devel libxcb-xkb-devel libxcb-icccm4-devel libxcb-image0-devel libxcb-keysyms1-devel libxcb-util0-devel libxcb-xinerama0-devel libxcb-xkb-devel libxkbcommon-x11-devel libxcb-cursor-devel
   ```

   Arch Linux:
   ```bash
   sudo pacman -S base-devel pkg-config libx11 libxcb xcb-util xcb-util-image xcb-util-keysyms xcb-util-wm
   ```

## Building and Running

### Development

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/ancestrum.git
   cd project-ancestrum
   ```

2. Run in development mode:
   ```bash
   cargo run
   ```

### Production

1. Build the release version:
   ```bash
   cargo build --release
   ```

2. The executable will be created at:
   - Windows: `target/release/ancestrum.exe`
   - macOS/Linux: `target/release/ancestrum`

3. Run the executable:
   - Windows: Double-click `ancestrum.exe` or run from command line
   - macOS: Open Terminal and run `./target/release/ancestrum`
   - Linux: Open Terminal and run `./target/release/ancestrum`

## Development


### Adding New Features
1. Create a new branch for your feature
2. Implement the changes
3. Run tests and ensure everything works
4. Submit a pull request

## Troubleshooting

### Common Issues

#### Windows
- If you get linker errors, ensure Visual Studio Build Tools are installed correctly
- If you get "link.exe not found" error, check your PATH environment variable

#### macOS
- If you get "framework not found" errors, ensure Xcode Command Line Tools are installed
- If you get "pkg-config not found", install it via Homebrew

#### Linux
- If you get "X11/Xlib.h not found", install the required development packages
- If you get "pkg-config not found", install the pkg-config package

## License

This project is licensed under the [MIT License](LICENSE).
