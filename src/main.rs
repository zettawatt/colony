// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

mod config;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    // Load the config file
    let (mut config, initialized): (config::Config, bool) = config::read_config();

    // Start the UI
    let ui = ColonyUI::new()?;
    let ui_handle = ui.as_weak();

    // Set the initial values of the configuration fields
    ui.global::<ConfigData>().set_download_path(config.downloads_path.clone().into());
    ui.global::<ConfigData>().set_data_path(config.data_path.clone().into());
    ui.global::<ConfigData>().set_password_timeout(config.password_timeout as i32);
    if !initialized {
        let ui = ui_handle.unwrap();
        ui.set_initialized(false);
    };

    /////////////////////////////////////////////
    // Colony Installation Callbacks
    /////////////////////////////////////////////
    // Go to next setup page
    ui.global::<SetupData>().on_inc_setup_step({
        let ui = ui_handle.unwrap();
        move || {
            ui.set_setup_step(ui.get_setup_step() + 1);
        }
     });
 
    // Go to previous setup page
    ui.global::<SetupData>().on_dec_setup_step({
        let ui = ui_handle.unwrap();
        move || {
            ui.set_setup_step(ui.get_setup_step() - 1);
        }
     });
 
    // Set password button
    ui.global::<SetupData>().on_set_password({
        let ui = ui_handle.unwrap();
        move |password1, password2| {
            let result: bool = config::initialize_password(password1.to_string(), password2.to_string());
            ui.global::<SetupData>().set_password_result(result);
        }
     });
 
    // Finish Setup
    ui.global::<SetupData>().on_finish_setup({
        let ui = ui_handle.unwrap();
        move || {
            ui.set_initialized(true);
        }
     });
 
    /////////////////////////////////////////////
    // Colony Configuration Tab Callbacks
    /////////////////////////////////////////////
    // Save the configuration fields
    ui.global::<ConfigData>().on_save_config({
        move |download_path, data_path, password_timeout| {
            config.set_config(download_path.to_string(), data_path.to_string(), password_timeout.to_string().parse::<u64>().unwrap());
        }
     });
 
    ui.run()?;

    Ok(())
}
