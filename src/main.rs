// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

mod config;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    // Load the config file
    let mut config: config::Config  = config::read_config().unwrap();

    // Start the UI
    let ui = ColonyUI::new()?;

    // Set the initial values of the configuration fields
    ui.global::<ConfigData>().set_download_path(config.downloads_path.clone().into());
    ui.global::<ConfigData>().set_data_path(config.data_path.clone().into());
    ui.global::<ConfigData>().set_password_timeout(config.password_timeout as i32);

    // Save the configurattion fields
    ui.global::<ConfigData>().on_save_config({
       move |download_path, data_path, password_timeout| {
           config.set_config(download_path.to_string(), data_path.to_string(), password_timeout.to_string().parse::<u64>().unwrap());
       }
    });

    ui.run()?;

    Ok(())
}
