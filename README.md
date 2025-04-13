# Ancestrum - Family Tree Visualizer

A modern family tree visualization application built with Rust and Vue 3.

## Project Structure

```
project-ancestrum/
├── backend/           # Rust backend
│   ├── src/          # Rust source code
│   └── Cargo.toml    # Rust dependencies
│
├── frontend/         # Vue frontend
│   ├── src/          # Vue source code
│   ├── package.json  # Node dependencies
│   └── vite.config.ts # Build configuration
```

## Development Setup

### Backend (Rust)

1. Navigate to the backend directory:
```bash
cd backend
```

2. Build and run the backend server:
```bash
cargo run
```

The backend server will run on http://localhost:8000

### Frontend (Vue 3)

1. Navigate to the frontend directory:
```bash
cd frontend
```

2. Install dependencies:
```bash
npm install
```

3. Start the development server:
```bash
npm run dev
```

The frontend will be available at http://localhost:3000

## Features

- Family tree visualization
- Add, edit, and delete family members
- Save and load family trees
- Modern, responsive UI
- Cross-platform support

## Technology Stack

- Backend: Rust with Warp web framework
- Frontend: Vue 3 with TypeScript
- Build Tools: Vite, Cargo
- State Management: Pinia
- Routing: Vue Router

## Prerequisites

### All Platforms
- Rust (latest stable version)
- Node.js (v16 or later)
- npm or yarn

### Linux
- `libwebkit2gtk-4.0-dev`
- `build-essential`
- `curl`
- `wget`
- `libssl-dev`
- `libgtk-3-dev`
- `libayatana-appindicator3-dev`
- `librsvg2-dev`

### macOS
- Xcode Command Line Tools
- `libiconv`

## Installation

1. Install Tauri CLI:
```bash
cargo install tauri-cli
```

2. Install platform-specific dependencies:

### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

### macOS
```bash
xcode-select --install
brew install libiconv
```

## Development

1. Clone the repository:
```bash
git clone https://github.com/yourusername/project-ancestrum.git
cd project-ancestrum
```

2. Install dependencies:
```bash
npm install
```

3. Start development server:
```bash
cargo tauri dev
```

## Building

To build the application for your platform:

```bash
cargo tauri build
```

This will create platform-specific installers in the `src-tauri/target/release` directory.

## Supported Platforms

- Windows (.msi)
- macOS (.app, .dmg)
- Linux (.deb)

## Data Storage

The application stores data in platform-specific locations:

- Windows: `%APPDATA%\com.ancestrum\project-ancestrum`
- macOS: `~/Library/Application Support/com.ancestrum/project-ancestrum`
- Linux: `~/.local/share/com.ancestrum/project-ancestrum`

## License

This project is licensed under the [MIT License](LICENSE).
