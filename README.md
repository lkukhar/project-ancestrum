# Ancestrum

A desktop application for managing family trees, built with Vue 3 and Tauri.

## Prerequisites

- [Node.js](https://nodejs.org/) (v16 or later)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)


## Setup

1. Install dependencies:
```bash
# Install Node.js dependencies
npm install

# Install Rust dependencies
cargo build
```

2. Configure environment:
```bash
# Create a .env file in the root directory
cp .env.example .env
```

## Development

Run the development server:
```bash
npm run tauri dev
```

This will:
- Start the Vite development server
- Build and run the Tauri application
- Open the application window
- Enable hot-reloading for both frontend and backend changes

## Testing

Run the test suite:
```bash
npm test
```

## Building

Create a production build:
```bash
npm run tauri build
```

This will create platform-specific binaries in the `src-tauri/target/release` directory.

## Features

- Family tree management
- Person profile creation and editing
- Relationship tracking
- Data persistence
- Cross-platform support

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

MIT License - see [LICENSE](LICENSE) for details
