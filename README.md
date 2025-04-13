# Ancestrum - Family Tree Visualizer

A modern, cross-platform family tree visualization application built with Rust, Tauri, and Vue.js.

## Features

- Create and manage family trees
- Add, edit, and delete family members
- Visualize family relationships
- Store family data locally
- Cross-platform desktop application
- Modern, responsive UI


## Prerequisites

- [Node.js](https://nodejs.org/) (v16 or later)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Git](https://git-scm.com/downloads)

### Platform-Specific Requirements

#### Windows
- Microsoft Visual Studio C++ Build Tools
- WebView2

```bash
winget install Microsoft.VisualStudio.BuildTools
# or download from https://visualstudio.microsoft.com/visual-cpp-build-tools/
```

#### macOS
- Xcode Command Line Tools

```bash
xcode-select --install
```

#### Linux
- Essential build tools and WebKit2GTK

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# Fedora
sudo dnf install webkit2gtk4.0-devel openssl-devel curl wget libappindicator-gtk3-devel librsvg2-devel

# Arch
sudo pacman -S webkit2gtk base-devel curl wget openssl gtk3 libappindicator-gtk3 librsvg
```

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/project-ancestrum.git
cd project-ancestrum
```

2. Install frontend dependencies:
```bash
cd src-frontend
npm install
```

3. Install Rust dependencies:
```bash
cd ../src-backend
cargo build
```

## Development

1. Start the frontend development server:
```bash
cd src-frontend
npm run dev
```

2. In a separate terminal, start the backend:
```bash
cd src-backend
cargo run
```

3. For Tauri development:
```bash
cd src-frontend
npm run tauri dev
```

## Building for Production

### Windows
```bash
cd src-frontend
npm run tauri build
```
The installer will be available in `src-tauri/target/release/bundle/`

### macOS
```bash
cd src-frontend
npm run tauri build
```
The `.app` bundle will be in `src-tauri/target/release/bundle/macos/`

### Linux
```bash
cd src-frontend
npm run tauri build
```
Various packages (deb, AppImage) will be in `src-tauri/target/release/bundle/`

## Testing

```bash
# Frontend tests
cd src-frontend
npm run test

# Backend tests
cd src-backend
cargo test
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For support, please open an issue in the GitHub repository.
