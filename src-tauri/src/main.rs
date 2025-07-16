// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::{Arg, Command};
use tracing::Level;
use tracing_subscriber::{filter, prelude::*};

#[tokio::main]
async fn main() {
    let matches = Command::new("colony-app")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A GUI frontend to the Autonomi network with a client side search engine")
        .arg(
            Arg::new("network")
                .short('n')
                .long("network")
                .value_name("NETWORK")
                .help("Network environment to connect to")
                .value_parser(["local", "alpha", "main"])
                .default_value("main"),
        )
        .get_matches();

    let network = matches.get_one::<String>("network").unwrap();

    let subscriber = tracing_subscriber::registry()
        .with(
            filter::Targets::new()
                .with_target("colonylib", Level::DEBUG) // DEBUG level for colonylib
                .with_target("colony", Level::DEBUG) // DEBUG level for colony
                .with_target("colony_app", Level::DEBUG) // DEBUG level for colony-app (includes dweb sidecar logs)
                .with_default(Level::ERROR),
        ) // ERROR level for other modules
        .with(tracing_subscriber::fmt::layer());

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Initialize tracing for logging
    //tracing_subscriber::fmt::init();

    // Run the Tauri application
    colony_app::run(network)
}
