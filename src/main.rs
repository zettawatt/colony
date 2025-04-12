// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

mod config;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    // Load the config file
    let _result: std::io::Result<config::Config> = config::read_config();

    // Start the UI
    let ui = ColonyUI::new()?;

//    ui.on_request_increase_value({
//        let ui_handle = ui.as_weak();
//        move || {
//            let ui = ui_handle.unwrap();
//            ui.set_counter(ui.get_counter() + 1);
//        }
//    });

    ui.run()?;

    Ok(())
}
