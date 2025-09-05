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
- 🖥️ **Cross-Platform** - Native applications for Linux, Windows, and macOS. Experimental Android app.

## 📦 Installation

### Download Pre-built Binaries

Download the latest release for your platform from the [Releases page](https://github.com/zettawatt/colony/releases) or search for and download the latest image from Colony!:

#### 🐧 Linux
- **AppImage**: `Colony_x.x.x_amd64.AppImage` - Portable application, no installation required. Mark it as executable, double click, and go.
- **DEB Package**: `Colony_x.x.x_amd64.deb` - For Debian/Ubuntu systems
- **RPM Package**: `Colony-x.x.x-1.x86_64.rpm` - For Red Hat/Fedora systems
- ⚠️ **Tauri Issues**: Some systems such as Arch linux and/or those running the Wayland compositor may have issues with the AppImage. If you encounter issues, buidling from source is recommended.

#### 🪟 Windows
- **MSI Installer**: `Colony_x.x.x_x64_en-US.msi` - Standard Windows installer
- ⚠️ **Security Notice**: The app is not signed with a Microsoft certificate. Windows will show a security warning. Click "More info" and then "Run anyway" to proceed.

#### 🍎 macOS
- **DMG Installer**: `Colony_x.x.x_aarch64.dmg` - For Apple silicon macOS systems
- **DMG Installer**: `Colony_x.x.x_x86_64.dmg` - For Intel-based macOS systems

#### 🤖 Android (Experimental, may be unstable)
- **APK**: `colony_x.x.x.apk` - For Android devices. Note that Colony is not available on Google Play due to cost and bureaucratic hurdles.

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

### Terminology
Before digging into how to use the app, it is important to answer this question: What is a pod? A pod is simply a container for descriptions of files on the Autonomi network. When a user uploads a file to Autonomi all metadata is stripped from the file. The only thing that remains is a long string of letters and numbers that tells you _where_ the file is, but not _what_ the file is.

In a pod you will say things like this _thing_ is a poem, it is written by Edgar Allen Poe, it has the title _The Raven_, etc. Every file you upload, you'll write descriptions like this in your pod(s), as will other users. When you share your pod(s) with others, they will get the same information that you attached to this file on the network.

In addition, pods can include references to other pods. The more pod references you can add in your pods and the more times your pods are referenced in other users' pods, the more links the network creates, the better your search results will be, and the more resiliant the network becomes.

At a high level this just means the more references you add and the more your pods are shared as references with others, the better.

### Initial setup
After installing and opening the application for the first time, you'll start at an introduction screen. You'll enter the following

- **Create Password** - You will be asked for this password each time you start the application. This is used to encrypt your private keys, so don't use a bad password!
- **Enter Seedphrase** - You can either generate a new seed phrase or enter an existing BIP39 compliant seed phrase. Make sure to write this down. If you ever want to restore your data or synchronize your app between multiple computers, you will need this seed phrase.
- **Confirm Seedphrase** - You will need to enter in the seedphrase from the previous screen again to ensure that you have written this phrase down.
- **Enter Wallet** - Enter in an Ethereum wallet private key. By default, a random wallet key will be generated. Note that this is not tied to your seed phrase and cannot be recreated. A better solution is to take the key from a true ETH wallet application. The easiest solution is to install [MetaMask]() in your web browser and use a private key from there. If you only want to use Colony with small amounts or use it for downloading only, the default key is fine.
- **Finish** - By default, the finish screen will have a check box selected to automatically add the Genesis Pod if not already added and sync the network. For first time users, leave this selected and click Finish. Wait for a few minutes for Colony to pull down the search indexes from the Autonomi network.

<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/initialization.gif" height="480" />
</p>

### Tab Layout
Colony uses tab navigation with each tab showing a different page.

**NOTE**: the screen captures below are a little dated, the UI is evolving very quickly. When in doubt, trust the text!

#### Colony App Info and Donation Addresses
Clicking on the Colony logo in the top left corner will display libraries used, application information, and crypto donation addresses to support Colony. If you like Colony, please donate BTC, ETH, USDC, or AUTONOMI tokens. Colony doesn't track you, does not display ads, does not censor, and isn't manipulating your search results like big tech companies do. Show your support and help develop the internet we all deserve!

Donation addresses:
- **BTC**: `bc1qp005au38ktl2zmhetsv223gld0sn3w456lkavw`
- **ETH/USDC/AUTONOMI**: `0xc6e3a7a770656B8473DedCc3d4565b6D507afACE`

#### Theme Switcher
The icon in the top right will select the theme. By default this is set to 'Auto' which will attempt to follow the system theme. However, depending on the OS, this is very error prone due to limitations of the underlying GUI toolkit. Clicking on this icon will cycle the color theme between Light, Dark, and Auto.

#### Searching for and downloading content
After entering your password, the search tab will open by default. If you hit enter without typing anything in or if you click the 'Browse' button, it will pull up available files on the network. Typing some text in the search bar will search the metadata for any objects that match your query, similar to your standard search engine on the WWW.

<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/search_screen.gif" height="480" />
</p>

To display more information about an object, click on the description or name. This will pull up a dialog displaying all metadata with an option to download the file. Clicking on the address will copy it to the clipboard. There are 3 types of objects:

- **files** - these have a down arrow icon on the far left of the row. Clicking on the arrow icon will automatically start downloading the file.
- **pods** - there will be no icon on the far left of the row for these. They are either your pods or pods that the search mechanism encountered. These can be ignored for now (functionality planned!).
- **dweb sites** - these will have a globe icon on the far left of the row. Clicking on the download link for a dweb site will automatically open the page in a web browser. These are web applications and sites that are hosted entirely on Autonomi. Very cool stuff. Be on the lookout for more of these as they come online!

<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/status_page.gif" height="480" />
</p>

Downloads can take some time depending on your network connection and file size.

#### Transfer Status Screen
The status tab shows the upload and download progress. While a file is being uploaded or downloaded, a spinner will show on the left side. On completion it will display a green checkmark while on fail, it will show a red 'X' icon.

#### Wallet management
In the wallet tab, you can add, rename, remove, and switch wallets. The active wallet is what is used to pay for uploads to Autonomi and also as the key for dweb sites that you may interact with.

<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/wallet.gif" height="480" />
</p>

#### Application configuration
In the configuration tab, you can change your download directory, password, and also the theme. Note that using auto dark/light mode can cause some bizarre behavior when switching between themes. It is much more reliable to select either Light or Dark and stick with it.

<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/configuration.gif" height="480" />
</p>

#### File Management
This tab is used to manage downloads, uploads, and pods. This tab is split into 3 sub pages:
- **Downloads** - lists completed downloads
- **Uploads** - upload files and lists completed uploads
- **My Pods** - add, modify, and remove pods

##### Downloads
Just as you would think, this page lists all of the files that have been downloaded. Clicking on these will open the file with the default system application. Depending on the binary and OS, the application being opened may be different. Older files uploaded to Autonomi are directories (i.e. archives). This can cause issues with the automatic file type detection. If auto file opening fails, go to the downloads directory as specified in the configuration screen to open them from your file browser application.

##### Uploads
See uploaded files and upload new files to the network. Check the status tab to see the progress of uploading files.

Clicking on the 'Upload New File' button will pull up a file picker window where you can select a file. If you've got the ETH and AUTONOMI tokens available in your active wallet, the file will upload to the network. After it completes, you'll see a notification at the bottom of the screen.

<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/upload_file.gif" height="480" />
</p>

##### My Pods
The My Pods page enables managing your pods, pod references, and attaching metadata to files in your pods.

###### Syncing from the Autonomi network
The 'Sync' button will pull pods from the Autonomi network. This will lock up the window while it fetches all of the metadata from the network. Run Sync any time you want to fetch the latest metadata from the network. Depending on the size of the pods, how many pod references you have, and your network speed, this operation will take some time. One note here is that a couple of the low level datatypes in Autonomi are still not rock solid. If you perform a Sync and do not see changes that you expect or if you don't see anything after the initial install, hit the Sync button again, possibly several times, until the data populates. This is not a Colony issue, but rather an Autonomi issue that will be resolved in the future.

Note it will throw a warning at this phase warning you about potential corruption if you haven't uploaded. If you have never uploaded to the network, there is no concern here. If you have, make sure you always click 'Upload All Pods' before you hit sync if you want to keep your changes. During the 'sync' operation, it will pull down all of your pods from the Autonomi network, so if you have local modifications, they could be lost.

**Tip** - sync operations are much faster if you have uploaded your pods to the network. This is because the first step is to pull your pods from the network and it takes much longer to search the network for non-existant data than to download existing data.

###### Add pod reference
After the setup step, Colony will leave you with a 'default' pod and a pod reference to the 'Genesis Pod' which contains the initial files uploaded to the Autonomi network. When you find other users' pod addresses, you can add these to the 'default' pod or create a new pod by clicking the 'Create New Pod' button. Click the pod's pen icon to edit. In the edit dialog, click on the and clicking on the 'Add Pod Ref' button to enter the pod reference address. When done editing the pod, click 'Save Pod'. Saving the pod will write it to your local disk and be uploaded to the network once you click the "Upload All Pods" button.

You can add as many pod references to your pod as you wish. If you want to share your pod with a friend, simply send them the address and they will perform the same steps.

<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/pod_management.gif" height="480" />
</p>

###### Adding metadata and syncing to Autonomi
To share uploaded files and add metadata to them, click the pencil icon on the selected pod, and click the transfer button to add this file to the pod. Then click the pencil icon to edit the metadata. There is a drop down at the top providing templates for various file types. Then enter in the relevant information in the fields.

You can also add files that were uploaded to Autonomi using the ant CLI or other application by clicking on the 'Add Autonomi File' button in the Editing Pod dialog. This will bring up a dialog where you can paste the Autonomi address of the object you want to describe.

The text looks rather complicated, but it isn't too hard once you've done a few. In simplified terms, the left side `"schema:blah"` statement specifies what type of information you should enter in the right hand text entry box. For example, if you are uploading a book and the left side says `"schema:author"`, in the right hand side you'd put the author's name in the text entry box.

Note that there is some magic that happens under the hood here. Several fields will be filled in automatically for files that you uploaded when you hit 'save', so you don't have to worry about them:
- `schema:contentSize`
- `schema:name`
- `schema:encodingFormat`

For files being added using the 'Add Autonomi File' button, these fields will need to be filled in manually as Colony has no direct information about these files. The `schema:contentSize` is the size of the file in bytes.

Any fields that are left with the default values from the template will not be included in the final result.

The entries that are the least self explanatory are `"schema:alternateName"` and `"schema:name"`. The rule here is that "name" is always the name of the file you're uploading, while "alternateName" is an optional human readable name. So if you upload a book with a filename "wizrdOz.pdf", set the "alternateName" to "The Wizard of Oz".

You can also edit metadata for items by selecting them and clicking the pencil icon that appears.

Although tempting, do not skip this step! Your file is only as valuable as the metadata that you put in here. If its wrong or lacking information, no one will be able to find your stuff! We will soon be improving the data entry mechanism here, we just ran out of time for the initial release!

Once you're happy, save the entry. You can make as many pods and upload as many files as you wish. Once everything is to your liking, hit the 'Upload All Pods' button to upload it to the network.

<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/metadata_entry.gif" height="480" />
</p>

This process is ok for individual file uploads, but becomes very tedious if you're doing more than a few of these. If you want to upload a bunch of stuff at once, check out the bulk uploader and downloader commands in [colony-utils](https://github.com/zettawatt/colony-utils). This automates the download, metadata curation, and Autonomi upload process for you.

###### Uploading Pods
After adding pod references or metadata for uploaded files, clicking the "Upload All Pods" button will push your pods to the Autonomi network. This step _is_ optional. Colony can run in download only mode with the pods saved to your local disk, requiring no ETH or AUTONOMI tokens to use. However, uploading your pods has several benefits:
- **Recovery** - With your seed phrase, you can recover all of your pods and access all files you've uploaded to the network. If your computer breaks or is lost reentering your seed phrase on boot enables you to recover everything you've had
- **Portability** - If you want to run Colony on another machine and have a common setup, uploading to Autonomi allows you to synchronize between multiple devices
- **Sync Speed** - one of the longest operations in sync is fetching your pods. Fetching non-existant data is very time consuming vs fetching data that exists
- **Sharing** - You are unable to share your pods if you have not uploaded them to the network.

The cost to upload several pods is very minimal (typically several US cents per pod) and once created can be updated for free.

If you have uploaded your pods in the past, ensure that you upload them every time before you run a Sync. This is because Sync will fetch your pods from the network and overwrite whatever you have done locally.

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
- **BTC**: `bc1qp005au38ktl2zmhetsv223gld0sn3w456lkavw`
- **ETH/AUTONOMI/USDC**: `0xc6e3a7a770656B8473DedCc3d4565b6D507afACE`

