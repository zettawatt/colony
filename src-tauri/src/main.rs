// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing::Level;
use tracing_subscriber::{filter, prelude::*};

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::registry()
        .with(
            filter::Targets::new()
                .with_target("colonylib", Level::DEBUG) // INFO level for colonylib
                .with_target("colony", Level::DEBUG) // INFO level for main.rs
                .with_default(Level::ERROR),
        ) // ERROR level for other modules
        .with(tracing_subscriber::fmt::layer());

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Initialize tracing for logging
    //tracing_subscriber::fmt::init();

    // Run the Tauri application
    colony::run()
}
