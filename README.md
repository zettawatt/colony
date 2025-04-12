<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/ui/images/logo-192x192.png" height="96" />
 <h1 align="center">
  colony
 </h1>
</p>

colony is a filesharing app and search engine for the [Autonomi](https://autonomi.com) network.

It is written in Rust and uses Slint as it UI framework.

Right now it is simply a Slint skeleton and doesn't do anything functional except create an initial config file. Work is ongoing.

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
