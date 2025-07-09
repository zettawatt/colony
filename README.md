<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/static/splash_screen.png" height="250" />
 <img src="https://img.shields.io/github/issues/zettawatt/colony?style=for-the-badge" />
 <img src="https://img.shields.io/github/actions/workflow/status/zettawatt/colony/ci.yml?style=for-the-badge&logo=github&logoColor=white&label=Build" />
 <img src="https://img.shields.io/github/actions/workflow/status/zettawatt/colony/release.yml?style=for-the-badge&logo=github&logoColor=white&label=Release" />
</p>

# 🌐 Colony GUI

**A user-friendly desktop application for the [Autonomi](https://autonomi.com) network**

Colony is a cross-platform GUI application that makes it easy to upload, download, share, and search for files on the decentralized Autonomi network. Powered by the [colonylib](https://crates.io/crates/colonylib) Rust crate, Colony provides an intuitive interface for managing your data on a censorship-resistant, permanent storage network.

## ✨ Features

- 🔐 **Secure Wallet Management** - BIP39 seed phrase generation and secure key storage
- 📁 **File Upload/Download** - Easy drag-and-drop file management with cost estimation
- 🔍 **Metadata Search** - Client-side search using RDF/SPARQL for rich data discovery
- 🌐 **Pod Management** - Create and manage metadata pods for organizing and sharing content
- 🔗 **Decentralized Sharing** - Share files and metadata without central servers
- 🖥️ **Cross-Platform** - Native applications for Linux, Windows, and macOS

## 📦 Installation

### Download Pre-built Binaries

Download the latest release for your platform from the [Releases page](https://github.com/zettawatt/colony/releases):

#### 🐧 Linux
- **AppImage**: `Colony-x.x.x.AppImage` - Portable application, no installation required. Mark it as executable, double click, and go.
- **DEB Package**: `colony-app_x.x.x_amd64.deb` - For Debian/Ubuntu systems
- **RPM Package**: `colony-app-x.x.x.x86_64.rpm` - For Red Hat/Fedora systems

#### 🍎 macOS
- **DMG**: `Colony_x.x.x_aarch64.dmg` - Drag to Applications folder
- ⚠️ **Security Notice**: The app is not signed with an Apple Developer certificate. You may need to right-click and select "Open" the first time, or go to System Preferences > Security & Privacy to allow the app to run.

#### 🪟 Windows
- **MSI Installer**: `Colony_x.x.x_x64_en-US.msi` - Standard Windows installer
- ⚠️ **Security Notice**: The app is not signed with a Microsoft certificate. Windows may show a security warning. Click "More info" and then "Run anyway" to proceed.

### Network Options

Colony supports multiple Autonomi networks:
- **Main Network** (default) - The production Autonomi network
- **Alpha Network** - For testing on the 'Alpha' network
- **Local Network** - For local development and testing

Use the `--network` flag when running from command line to select the alpha or local network options:
```bash
colony-app --network alpha
```

## 🏗️ Architecture

Colony is built on a modular architecture that separates concerns between the GUI and core functionality:

- **Frontend**: Modern web UI built with [Svelte](https://svelte.dev/) and [Tauri](https://tauri.app/)
- **Backend**: Rust-based core using [colonylib](https://crates.io/crates/colonylib) for Autonomi network operations
- **Storage**: Client-side RDF graph database for metadata and search capabilities
- **Network**: Direct integration with [Autonomi](https://autonomi.com) for decentralized file storage

### Related Projects

- **[colonylib](https://crates.io/crates/colonylib)** - The core Rust library that handles all Autonomi network operations, pod management, and metadata processing
- **[colony-utils](https://github.com/zettawatt/colony-utils)** - Command-line interface that implements the same colonylib functionality for power users and automation

## 🔧 How It Works

Colony implements a decentralized metadata system using the concept of "pods" - collections of RDF metadata about files stored on Autonomi:

1. **File Upload**: When you upload a file, Colony stores it on Autonomi and creates RDF metadata describing the file
2. **Pod Creation**: Metadata is organized into "pods" - publicly readable collections that can be shared with others
3. **Discovery**: Users share pod addresses to give others access to their file metadata and discover new content
4. **Search**: All metadata is stored locally in an RDF graph database, enabling powerful SPARQL-based searches
5. **Decentralized Network**: No central servers - everything runs on your device and the Autonomi network

The system leverages [RDF](https://www.w3.org/RDF/) and [SPARQL](https://en.wikipedia.org/wiki/SPARQL) technologies to create a rich, interconnected web of metadata that grows organically as users share and discover content.

## 🚀 Development Environment Setup

### Prerequisites

- **Rust** - Install from [rustup.rs](https://rustup.rs/)
- **Node.js** - Install LTS version from [nodejs.org](https://nodejs.org/)
- **Git** - For cloning the repository

### Setup Steps

1. **Clone the repository**
   ```bash
   git clone https://github.com/zettawatt/colony.git
   cd colony
   ```

2. **Install frontend dependencies**
   ```bash
   npm install
   ```

3. **Run in development mode**
   ```bash
   npm run tauri dev
   ```

### Building for Production

```bash
# Build the frontend
npm run build

# Build the Tauri application
npm run tauri build
```

### Recommended IDE Setup

- **[VS Code](https://code.visualstudio.com/)** with the following extensions:
  - [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 📸 Screenshots

Main search interface:
<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/search.png" height="480" />
</p>

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.

## 💝 Support

Support Colony development by donating:
- **ETH/ANT**: `0xc6e3a7a770656B8473DedCc3d4565b6D507afACE`
