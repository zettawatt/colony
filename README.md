<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/ui/images/splash_screen.png" height="250" />
 <img src="https://img.shields.io/github/issues/zettawatt/colony?style=for-the-badge" />
 <img src="https://img.shields.io/github/actions/workflow/status/zettawatt/colony/rust.yml?style=for-the-badge&logo=github&logoColor=white&label=Build" />
</p>

## Why do I care?

With [Autonomi](https://autonomi.com), we are finally free to host content forever, without fears of censorship or link rot. The problem is, how do you easily share that data with your friends? Or search for things that interest you? Or remember where all of your data is stored? Colony is an easy to use GUI that solves these problems and brings [Autonomi](https://autonomi.com) to the masses.

## What is Colony?

Colony is a native cross-platform (Linux, Windows, Mac) desktop application written in [Rust](https://www.rust-lang.org/) and using the [Slint](https://slint.dev/) GUI framework. It enables users to upload, download, share, and search for files on the [Autonomi](https://autonomi.com) network. There are no servers, no oracles, and no shared databases of information.

[Autonomi](https://autonomi.com) is modeled after the natural process of ‘emergence’. Colony takes this concept and applies it to metadata. Each file uploaded to Autonomi by Colony will write metadata in [RDF](https://www.w3.org/RDF/) about that file and other files to a publicly readable scratchpad. This scratchpad is called a ‘pod’, following the semantics of the [Solid](https://solidproject.org/) project. A user can break up metadata into as many or as few pods as they wish. The address to the user’s pods is listed in a ‘pod list’ publicly readable scratchpad. Just like pods, the user can have multiple pod lists. To share files with a friend, a user simply gives them the address to their pod list or pod lists. Now the friend has access to all of the metadata for all the files referenced in the pods in that pod list. A pod list doesn’t just list the user’s pods, it can also list other pod lists from others they have collected. With only a few degrees of separation between people, users will very quickly build up an index of all files, metadata, and most importantly, how these files are related to each other.

Users will download all of the pods and pod lists and handle searching fully on the client, no server required. Because the pods are written in RDF (using the [turtle](https://en.wikipedia.org/wiki/Turtle_(syntax)) syntax), Colony will leverage the [SPARQL](https://en.wikipedia.org/wiki/SPARQL) querying language under the hood. To the average user, it will operate like any other search bar, but underneath it enables very complex interactive queries. For example, search for a song and it will contain an RDF description for the artist. In the file info window, click on the artist name to do a SPARQL query of all songs by that artist or by a particular album. Users can even type in a raw SPARQL query themselves. Or for the more adventurous, the RDF data is contained in simple files stored on the client, enabling other local applications or locally hosted AI agents to query. Eventually the goal is to have an API for external application interactions and support for mobile light clients using the user’s desktop for the compute and storage intensive search operations.

Because the user created the data for the pod and the pod lists in scratchpads, the metadata can be appended, modified, refactored, or deleted at 0 cost once created. The only cost is for additional pods. The user owns the data fully and is in control.

The end goal for Colony isn’t to be the end-all app for [Autonomi](https://autonomi.com). It is simply the first application using the metadata scheme above, setting up a framework for other apps to leverage in the future. If the app ecosystem on Autonomi leverages some common frameworks, we all benefit from the ability to seamlessly move between apps. This is the internet as it was meant to be. As it should be. As it will be.

## Current Status

Colony has the installation GUI in place that will create a configuration file, generate a BIP39 compliant 12 word seen phrase, and a main BLS secret key for generating pod file addresses, all stored in a password protected [cocoon](https://docs.rs/cocoon/latest/cocoon/index.html). Colony can query the balance from your provided ETH address. Next up is creating the pod infrastructure and support file uploads.

## Setup

1. Install Rust by following its [getting-started guide](https://www.rust-lang.org/learn/get-started).
   Once this is done, you should have the `rustc` compiler and the `cargo` build system installed in your `PATH`.
2. Clone this repo
2. Navigate to the cloned directory and build with `cargo`:
    ```
    cargo build
    ```
3. Run colony:
    ```
    cargo run
    ```

It is highly recommended to use an IDE for development, along with the [LSP-based IDE integration for `.slint` files](https://github.com/slint-ui/slint/blob/master/tools/lsp/README.md). You can also load this project directly in [Visual Studio Code](https://code.visualstudio.com) and install the [Slint extension](https://marketplace.visualstudio.com/items?itemName=Slint.slint).

## Screenshots

Main search page:
<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/screenshots/search.png" height="480" />
</p>

## Want to help?

Support Colony by donating ETH or ANT tokens here: 0xc6e3a7a770656B8473DedCc3d4565b6D507afACE
