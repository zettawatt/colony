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
- **AppImage**: `colony-app_x.x.x_amd64.AppImage` - Portable application, no installation required. Mark it as executable, double click, and go.
- **DEB Package**: `colony-app_x.x.x_amd64.deb` - For Debian/Ubuntu systems
- **RPM Package**: `colony-app-x.x.x-1.x86_64.rpm` - For Red Hat/Fedora systems

#### 🪟 Windows
- **MSI Installer**: `colony-app_x.x.x_x64_en-US.msi` - Standard Windows installer
- ⚠️ **Security Notice**: The app is not signed with a Microsoft certificate. Windows may show a security warning. Click "More info" and then "Run anyway" to proceed.

#### 🍎 macOS - BROKEN
- Due to the very stringent "security" measures taken by MacOS, we are unable to deliver binaries at this time. See below for how to build the Colony App binary from source.

### Network Options

Colony supports multiple Autonomi networks:
- **Main Network** (default) - The production Autonomi network
- **Alpha Network** - For testing on the 'Alpha' network
- **Local Network** - For local development and testing

Use the `--network` flag when running from command line to select the alpha or local network options:
```bash
colony-app --network alpha
```

## User Manual

### Initial setup
After installing and opening the application for the first time, you'll start at an introduction screen. You'll enter the following

- **Create Password** - You will be asked for this password each time you start the application. This is used to encrypt your private keys, so don't use a bad password!
- **Enter Seedphrase** - You can either generate a new seed phrase or enter an existing BIP39 compliant seed phrase. Make sure to write this down. If you ever want to restore your data or synchronize your app between multiple computers, you will need this seed phrase.
- **Enter Wallet** - Enter in an Ethereum wallet private key. The easiest solution is to install [MetaMask]() in your web browser and use a private key from there. If you only want to test Colony or use it for downloading only, you can skip this for now.

<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/initialization.gif" height="480" />
</p>

### Add pod reference
After the setup step, the first thing you should do is add a pod. Give it any name that you want, you can also rename it later if you wish. Once created, edit that pod and add a reference to the 'Colony Genesis Pod'. This pod essentially bootstraps you into the Colony metadata collection so you have something to search. The Genesis Pod address is as follows:

```
aaa518a2cf8260f6bebc769c16b8147ea215adf569696497b7fc1f250823d89a49990187e83fc0f9ae1cf3d44afb7dce
```

You can add as many pod references to your pod as you wish. If you want to share your pod with a friend, simply sent them the address and they will get everything.

### Syncing from the Autonomi network
And finally hit the 'Sync' button. This will lock up the window for a few moments while it fetches all of the metadata from the network. Note it will throw a warning at this phase warning you about potential corruption if you haven't uploaded. If you have never uploaded to the network, there is no concern here. If you have, make sure you always click 'Upload All Pods' before you hit sync if you want to keep your changes. During the 'sync' operation, it will pull down all of your pods from the Autonomi network, so if you have local modifications, they could be lost.

<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/pod_management.gif" height="480" />
</p>


### Searching for content
Now you're ready to search! Click on the search tab. If you hit enter without typing anything in or if you hit 'Browse', it will pull up available files on the network. Typing some text in the search bar will search the metadata for any objects that match your query, similar to your standard search engine on the WWW.

<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/search_screen.gif" height="480" />
</p>


### Downloading content and viewing Autonomi web apps
In the search results you'll see an info icon which displays more information about the object, a download icon which downloads that file, the name of the file, its size, and finally the Autonomi address for the file. If the size field is filled out, it is almost certainly a file you can download. There 'Unknown' types are typically one of 2 things:

- **pods** - these are either your pods or pods that the search mechanism encountered. These can be ignored, but you can use the info icon to get their address.
- **dweb sites** - clicking on the download link for a dweb site will automatically open the page in a web browser. These are web applications and sites that are hosted entirely on Autonomi. Very cool stuff. Be on the lookout for more of these as they come online!

<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/status_page.gif" height="480" />
</p>

Downloads can take some time depending on your network connection.

### Application configuration
In the configuration tab, you can change your download directory, password, and also the theme:

<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/configuration.gif" height="480" />
</p>

### Wallet management
In the wallet tab, you can add, rename, remove, and switch wallets. The active wallet is what is used to pay for uploads to Autonomi and also as the key for dweb sites that you may interact with.

<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/wallet.gif" height="480" />
</p>

### Uploading files to Autonomi
Finally, we come to uploads. Back on the File Management screen, click on the 'Uploads' tab, and then the 'Upload New File'. This will pull up a file picker window where you can select a file. If you've got the ETH and ANT tokens available in your active wallet, the file will upload to the network. After it completes, you'll see a notification at the bottom of the screen.

<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/upload_file.gif" height="480" />
</p>

### Adding metadata and syncing to Autonomi
To share this newly uploaded file and add metadata to it, go back to the Pod Management tab on the left side of the screen, click the modify button on the selected pod, and click the transfer button to add this file to the pod. Then click 'Edit' to edit the metadata. There is a drop down at the top providing templates for various file types. Then enter in the relevant information in the text box.

The text looks rather complicated, but most of it you don't care about. In simplified terms, the left side `"schema:blah"` statement specifies what type of information you should enter in the right hand `" "` set of characters on that line. For example, if you are uploading a book and the left side says `"schema:Author"`, the left hand side, you'd put the author's name inside the `" "` characters. You can also take the starting point and feed it into a chatbot and ask it to fill in more information for this JSON-LD entry and it typically figures it out for you.

Note that there is some magic that happens under the hood here. Several fields will be filled in automatically when you hit 'save', so you don't have to worry about them:
- `schema:contentSize`
- `schema:name`
- `schema:encodingFormat`

The entries that are the least self explanatory are `"schema:alternateName"` and `"schema:name"`. The rule here is that "name" is always the name of the file you're uploading, while "alternateName" is an optional human readable name. So if you upload a book with a filename "wizrdOz.pdf", set the "alternateName" to "The Wizard of Oz".

Although tempting, do not skip this step! Your file is only as valuable as the metadata that you put in here. If its wrong or lacking information, no one will be able to find your stuff! We will soon be improving the data entry mechanism here, we just ran out of time for the initial release!

Once you're happy, save the entry. You can make as many pods and upload as many files as you wish. Once everything is to your liking, hit the 'Upload All Pods' button to upload it to the network.

<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/metadata_entry.gif" height="480" />
</p>

This process is ok for individual file uploads, but becomes very tedious if you're doing more than a few of these. If you want to upload a bunch of stuff at once, check out the bulk uploader and downloader commands in [colony-utils](https://github.com/zettawatt/colony-utils). This automates the download, metadata curation, and Autonomi upload process for you.

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

1. **File Upload**: When you upload a file, Colony stores it on Autonomi and you create RDF metadata describing the file
2. **Pod Creation**: Metadata is organized into "pods" - publicly readable collections that can be shared with others
3. **Discovery**: Users share pod addresses to give others access to their file metadata and discover new content
4. **Search**: All metadata is stored locally in an RDF graph database, enabling powerful SPARQL-based searches
5. **Decentralized Network**: No central servers - everything runs on your device and the Autonomi network

The system leverages [RDF](https://www.w3.org/RDF/) and [SPARQL](https://en.wikipedia.org/wiki/SPARQL) technologies to create a rich, interconnected web of metadata that grows organically as users share and discover content.

## 🚀 Development Environment Setup

### Prerequisites

- **Rust** - Install from [rustup.rs](https://rustup.rs/)
- **Node.js** - Install Node 20.19.0 [nodejs.org](https://nodejs.org/download/release/v20.19.0/)
- **Git** - For cloning the repository
- **System Dependencies** See Tauri's reference docs about this [tauri.app](https://tauri.app/start/prerequisites/#system-dependencies)

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

3. **Platform-specific configuration**
     Before running the application, we rely on a rocksdb to create the graph database. If you run into trouble, you may need to ensure that c++ dependencies are setup correctly. 

   - #### 🍎 **macOS: Configure clang (C++)**
      <details>
         <summary>Setting up C++ ENV Variables</summary>
         <br>

         [target.aarch64-apple-darwin.env]
         SDKROOT = "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk"
         CPLUS_INCLUDE_PATH = "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/c++/v1"

         [target.x86_64-apple-darwin.env]
         SDKROOT = "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk"
         CPLUS_INCLUDE_PATH = "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/c++/v1"
      </details>

   - #### 🪟 **Windows**
      <details> 
         <summary>Windows Libclang Setup</summary>
         <br>

         Step 1: Install LLVM

         Download and install LLVM from the official GitHub releases page:

         https://github.com/llvm/llvm-project/releases

         During installation:
         - Enable adding LLVM to your system PATH
         - If you skip this, you'll need to manually add it later

         Step 2: Set the `LIBCLANG_PATH` Environment Variable

         After installation, locate `libclang.dll`, typically at:
         C:\Program Files\LLVM\bin\libclang.dll
         
         ##### Temporary (PowerShell):
         $env:LIBCLANG_PATH="C:\Program Files\LLVM\bin"

      </details>

   - #### 🐧 **Linux**
      <details>
         <summary>Installing Clang (Ubuntu)</summary>
         <br>

         Step 1: Update
         sudo apt update

         Step 2: Install
         sudo apt install clang

      </details>

4. **Run in development mode**
   ```bash
   npm run tauri dev
   ```

### Building From Source

After you've setup the dev evironment, you can build from source by using the following command followed by your desired options:

Windows:
```bash
npm run tauri build
```

To build the MacOS app, you can do so like this:
```bash
npm run tauri build -- --bundles app
```

For more information, see Tauri's build documentation here [Tauri:Building](https://tauri.app/distribute/#distributing)

### Recommended IDE Setup

- **[VS Code](https://code.visualstudio.com/)** with the following extensions:
  - [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.

## 💝 Support

Support Colony development by donating:
- **ETH/ANT**: `0xc6e3a7a770656B8473DedCc3d4565b6D507afACE`

