# Ancestrum - Family Tree Visualizer

A modern desktop application for visualizing and managing family trees, built with Tauri, Vue 3, and Rust.

## Project Structure

```
project-ancestrum/
├── src-tauri/              # Tauri backend (Rust)
│   ├── api/                # API endpoints
│   ├── capabilities/       # Tauri capabilities
│   ├── icons/              # Application icons
│   ├── build.rs            # Build script
│   ├── Cargo.toml          # Rust dependencies
│   ├── lib.rs              # Library entry point
│   ├── main.rs             # Application entry point
│   └── tauri.conf.json     # Tauri configuration
├── src-frontend/           # Frontend code (Vue 3)
│   ├── app/                # Vue components
│   ├── vite.config.ts      # Vite configuration
│   ├── tsconfig.json       # TypeScript configuration
│   ├── tailwind.config.js  # Tailwind CSS configuration
│   └── postcss.config.js   # PostCSS configuration
├── package.json            # Project dependencies and scripts
└── LICENSE                 # MIT License
```

## Features

- Add, update, and delete family members
- Visualize family relationships
- Save and load family tree data
- Modern, responsive UI
- Cross-platform support (Windows, macOS, Linux)

## Prerequisites

### Common Requirements
- [Node.js](https://nodejs.org/) (v16 or later)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Platform-Specific Requirements

#### Windows
- [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (usually installed with Windows 11)

#### macOS
- Xcode Command Line Tools:
  ```bash
  xcode-select --install
  ```
- [Homebrew](https://brew.sh/) (recommended for package management)

#### Linux
- System dependencies:
  ```bash
  # Debian/Ubuntu
  sudo apt update
  sudo apt install -y \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

  # Fedora
  sudo dnf install -y \
    gcc \
    openssl-devel \
    gtk3-devel \
    webkit2gtk4.0-devel \
    libappindicator \
    librsvg2-devel

  # Arch Linux
  sudo pacman -S --needed \
    base-devel \
    curl \
    wget \
    openssl \
    gtk3 \
    webkit2gtk \
    libappindicator \
    librsvg
  ```

## Development Setup

1. Install dependencies:
```bash
npm install
```

2. Start the development server:
```bash
npm run tauri:dev
```

### Platform-Specific Development Notes

#### Windows
- Ensure you have the latest version of WebView2 installed
- If you encounter build errors, try running the following in an elevated PowerShell:
  ```powershell
  npm install --global --production windows-build-tools
  ```

#### macOS
- If you encounter permission issues with the camera or file system, you may need to:
  1. Open System Preferences > Security & Privacy
  2. Add the development build to the allowed applications
- For Apple Silicon (M1/M2) Macs, ensure you have Rosetta 2 installed:
  ```bash
  softwareupdate --install-rosetta
  ```

#### Linux
- If you encounter issues with the window not showing up, ensure your system has the required GTK and WebKit dependencies
- For Wayland users, you might need to set the following environment variable:
  ```bash
  export WAYLAND_DISPLAY=wayland-0
  ```

## Building for Production

```bash
npm run tauri:build
```

### Platform-Specific Build Notes

#### Windows
- The build will create an installer (.msi) in the `src-tauri/target/release` directory
- Signing the application is recommended for distribution

#### macOS
- The build will create a .app bundle and .dmg installer
- For distribution, you'll need an Apple Developer certificate for signing

#### Linux
- The build will create a .deb package (Debian/Ubuntu) or .AppImage
- For .deb packages, ensure you have the required dependencies installed

## Project Structure Details

### Frontend (`src-frontend/`)
- Built with Vue 3 and TypeScript
- Uses Vite as the build tool
- Styled with Tailwind CSS
- State management with Pinia

### Backend (`src-tauri/`)
- Rust-based backend using Tauri
- Graph-based data structure for family relationships
- Persistent storage using JSON files
- Thread-safe operations with Mutex

## Available Scripts

- `npm run dev` - Start the frontend development server
- `npm run build` - Build the frontend for production
- `npm run tauri:dev` - Start the Tauri development server
- `npm run tauri:build` - Build the Tauri application for production

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For support, please open an issue in the GitHub repository.
