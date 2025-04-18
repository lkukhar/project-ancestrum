# Ancestrum - Family Tree Application

A modern, cross-platform family tree application built with Tauri, Vue 3, and Rust.

## Features

- Create and manage family trees
- Add, edit, and delete family members
- Define relationships between family members
- Cross-platform support (Windows, macOS, Linux)
- Modern, responsive UI with Tailwind CSS
- Type-safe API with TypeScript and Rust

## Project Structure

```
ancestrum/
├── src-frontend/              # Frontend code (Vue 3 + TypeScript)
│   ├── app/
│   │   ├── components/        # Vue components
│   │   ├── services/          # API services
│   │   ├── store/             # Pinia store
│   │   ├── types/             # TypeScript types
│   │   ├── App.vue            # Root Vue component
│   │   └── main.ts            # Entry point
│   ├── public/                # Static assets
│   ├── index.html             # HTML template
│   ├── package.json           # Frontend dependencies
│   ├── tailwind.config.js     # Tailwind configuration
│   └── vite.config.ts         # Vite configuration
│
├── src-tauri/                 # Backend code (Rust)
│   ├── models/                # Data models
│   ├── commands.rs            # Tauri command handlers
│   ├── main.rs                # Entry point
│   ├── Cargo.toml             # Rust dependencies
│   └── tauri.conf.json        # Tauri configuration
│
├── package.json               # Root package.json
└── README.md                  # This file
```

## Prerequisites

### All Operating Systems

- [Node.js](https://nodejs.org/) (v18 or later)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Windows

1. Install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. Install [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

### macOS

1. Install Xcode Command Line Tools:
   ```bash
   xcode-select --install
   ```

### Linux

1. Install system dependencies:
   ```bash
   # Debian/Ubuntu
   sudo apt update
   sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

   # Arch Linux
   sudo pacman -S webkit2gtk base-devel curl wget openssl appmenu-gtk-module gtk3 libappindicator-gtk3 librsvg libvips

   # Fedora
   sudo dnf install webkit2gtk3-devel.x86_64 openssl-devel curl wget libappindicator-gtk3 librsvg2-devel
   ```

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/ancestrum.git
   cd ancestrum
   ```

2. Install dependencies:
   ```bash
   # Install Rust dependencies
   cargo install tauri-cli

   # Install Node.js dependencies
   npm install
   ```

## Development

### Running the Development Server

#### Windows
```bash
# Using PowerShell
npm run tauri:dev

# Using Command Prompt
npm run tauri:dev
```

#### macOS/Linux
```bash
# Using bash/zsh
npm run tauri:dev
```

### Building for Production

#### Windows
```bash
# Using PowerShell
npm run tauri:build

# Using Command Prompt
npm run tauri:build
```

#### macOS/Linux
```bash
# Using bash/zsh
npm run tauri:build
```

## Project Architecture

### Frontend (Vue 3 + TypeScript)

- **Components**: Reusable Vue components in `src-frontend/app/components/`
- **Store**: State management using Pinia in `src-frontend/app/store/`
- **Services**: API communication layer in `src-frontend/app/services/`
- **Types**: TypeScript type definitions in `src-frontend/app/types/`

### Backend (Rust)

- **Models**: Data structures in `src-tauri/models/`
- **Commands**: Tauri command handlers in `src-tauri/commands.rs`
- **State Management**: Thread-safe state using `Mutex` and `AppState`

## API Reference

### Person Management

- `add_person(name: string)`: Add a new person
- `get_person(id: string)`: Get person details
- `update_person(id, name, birth_date, death_date, gender, notes)`: Update person
- `delete_person(id: string)`: Delete a person

### Relationship Management

- `add_relationship(from_id, to_id, relationship)`: Add a relationship
- `get_relationships(id: string)`: Get all relationships for a person

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Tauri](https://tauri.app/) for the cross-platform framework
- [Vue.js](https://vuejs.org/) for the frontend framework
- [Tailwind CSS](https://tailwindcss.com/) for styling
- [Pinia](https://pinia.vuejs.org/) for state management
