// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::rc::Rc;
//use cocoon::Cocoon;
use config::SeedPhrase;
use slint::{ModelRc, VecModel, SharedString, Model};

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

    // Generate seed phrase button
    ui.global::<SetupData>().on_generate_seed_phrase({
        let ui = ui_handle.unwrap();
        move || {
            let seed_phrase: SeedPhrase = config::generate_seed_phrase();
            // A bunch of boilerplate to convert the Vec<String> to a Vec<SharedString>
            // and then to a VecModel<SharedString> and finally to a ModelRc<SharedString>
            // This is necessary because the UI needs a ModelRc<SharedString> to display the seed phrase
            let seed_phrase_vec: Rc<VecModel<SharedString>> = Rc::new(VecModel::from(
                seed_phrase.seed_words.into_iter().map(SharedString::from).collect::<Vec<_>>()
            ));
            let seed_phrase_modelrc = ModelRc::from(seed_phrase_vec);
            ui.global::<SetupData>().set_seed_phrase(seed_phrase_modelrc);
        }
    });

    // Check seed phrase validity button
    ui.global::<SetupData>().on_check_seed_phrase_validity({
        let ui = ui_handle.unwrap();
        move |seed_phrase| {
            // Convert the ModelRc<SharedString> type to a Vec<String>
            let seed_phrase_vec: Vec<String> = seed_phrase.iter().map(|s| s.to_string()).collect();
            let result: bool = config::check_seed_phrase_validity(seed_phrase_vec);
            ui.global::<SetupData>().set_check_seed_phrase_validity_result(result);
        }
     });
 
    // Compare seed phrase button
    ui.global::<SetupData>().on_compare_seed_phrase({
        let ui = ui_handle.unwrap();
        move |seed_phrase, confirmed_seed_phrase| {
            // Convert the ModelRc<SharedString> type to a Vec<String>
            let seed_phrase_vec: Vec<String> = seed_phrase.iter().map(|s| s.to_string()).collect();
            let confirmed_seed_phrase_vec: Vec<String> = confirmed_seed_phrase.iter().map(|s| s.to_string()).collect();
            let result: bool = config::compare_seed_phrase(seed_phrase_vec, confirmed_seed_phrase_vec);
            ui.global::<SetupData>().set_compare_seed_phrase_result(result);
        }
     });

     // Check ethereum ethereum private key button
     ui.global::<SetupData>().on_check_ethereum_private_key({
        let ui = ui_handle.unwrap();
        move |private_key| {
            let result: bool = config::check_ethereum_private_key(private_key.to_string());
            if result {
                ui.global::<SetupData>().set_ant_balance(0.0.to_string().into());
                ui.global::<SetupData>().set_eth_balance(0.0.to_string().into());
            }
            ui.global::<SetupData>().set_check_ethereum_private_key_result(result);
        }
     });
 
    // Finish Setup
    ui.global::<SetupData>().on_finish_setup({
        let ui = ui_handle.unwrap();
        move || {
            ui.set_initialized(true);
            //FIXME: need to create a cocoon here with the secrets
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
