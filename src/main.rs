// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::rc::Rc;
//use cocoon::Cocoon;
use config::SeedPhrase;
use slint::{ModelRc, VecModel, SharedString, Model};
pub const BAD_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

mod config;
mod data;
mod network;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {

    // Initialize the SecretData struct
    let secret_data = Rc::new(std::cell::RefCell::new(data::SecretData::from_mnemonic(BAD_MNEMONIC.to_string()).unwrap()));

    // Load the config file
    let initialized = config::check_config();
    let config = Rc::new(std::cell::RefCell::new(config::read_config()));

    // Start the UI
    let ui = ColonyUI::new()?;
    let ui_handle = ui.as_weak();

    // Setup worker to handle async network calls
    let network_worker = network::NetworkWorker::new(&ui);

    // Set the initial values of the configuration fields
    let download_path: String = config.borrow().get_downloads_path().clone();
    ui.global::<ConfigData>().set_download_path(download_path.into());
    let data_path: String = config.borrow().get_data_path().clone();
    ui.global::<ConfigData>().set_data_path(data_path.into());
    let password_timeout: i32 = config.borrow().get_password_timeout() as i32;
    ui.global::<ConfigData>().set_password_timeout(password_timeout);
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
            let result: bool = config::check_ethereum_private_key(private_key.clone().to_string());
            ui.global::<SetupData>().set_check_ethereum_private_key_result(result);
        }
     });
 
    // Finish Setup
    ui.global::<SetupData>().on_finish_setup({
        let ui = ui_handle.unwrap();
        let data_path = config.borrow().get_data_path();
        move || {
            ui.set_initialized(true);
            let seed_phrase_vec: Vec<String> = ui.global::<SetupData>().get_seed_phrase().iter().map(|s| s.to_string()).collect();
            let seed_phrase: String = seed_phrase_vec.join(" ");
            let mut secret_data = data::SecretData::from_mnemonic(seed_phrase).unwrap();
            secret_data.set_wallet(ui.global::<SetupData>().get_ethereum_private_key().to_string());
            let data_path_clone: String = data_path.clone();
            let data_path_full: String = data_path.clone() + "/secrets.db";
            let data_path_full_clone = data_path_full.clone();
            let mut secrets_file = std::fs::File::create(data_path_full).unwrap_or_else(
                |error| {
                    if error.kind() == std::io::ErrorKind::NotFound {
                        std::fs::create_dir_all(&data_path_clone).unwrap();
                        println!("Creating directory: {:?}", data_path_clone);
                        std::fs::File::create(data_path_full_clone).unwrap()
                    } else {
                        panic!("Problem creating the secrets file: {:?}", error);
                    }
                }
            );
            let password = ui.global::<SetupData>().get_password();
            secret_data.to_file(&mut secrets_file, password.as_str()).unwrap();
            ui.global::<WalletData>().set_wallet_address(secret_data.get_wallet().address().to_string().into());
        }
     });
 
    /////////////////////////////////////////////
    // Colony Configuration Tab Callbacks
    /////////////////////////////////////////////
    // Save the configuration fields
    ui.global::<ConfigData>().on_save_config({
        let config = Rc::clone(&config);
        move |download_path, data_path, password_timeout| {
            let download_path = download_path.to_string();
            let data_path = data_path.to_string();
            let password_timeout = password_timeout.to_string().parse::<u64>().unwrap();
            config.borrow_mut().set_config(download_path, data_path, password_timeout);
        }
    });

     // unlock the password
     ui.global::<ConfigData>().on_unlock_password({
        let ui = ui_handle.unwrap();
        let data_path = config.borrow().get_data_path().clone();
        //let data_path = config.get_data_path().clone();
        let secret_data = Rc::clone(&secret_data);
        move |password: SharedString| {
            let data_path_full: String = format!("{}/{}", data_path, "secrets.db");
            let mut file = std::fs::File::open(data_path_full).unwrap();
            ui.global::<ConfigData>().set_password_correct(data::SecretData::from_file(&mut file, password.as_str()).is_ok());
            if !ui.global::<ConfigData>().get_password_correct() {
                ui.global::<ConfigData>().set_password_status("Password is incorrect".into());
            } else {
                ui.global::<ConfigData>().set_password_status("".into());
            }
            *secret_data.borrow_mut() = data::SecretData::from_file(&mut file, password.as_str()).unwrap_or_else(|_error| {
                ui.global::<ConfigData>().set_password_status("Password is incorrect".into());
                data::SecretData::from_mnemonic(BAD_MNEMONIC.to_string()).unwrap()
            });
        }
     });

     // lock from password timeout
     ui.global::<ConfigData>().on_lock_password({
        let secret_data = Rc::clone(&secret_data);
        move || {
            *secret_data.borrow_mut() = data::SecretData::from_mnemonic(BAD_MNEMONIC.to_string()).unwrap();
        }
     });

    // Check password matches
    ui.global::<ConfigData>().on_check_password({
        let ui = ui_handle.unwrap();
        move |password1, password2| {
            let result: bool = config::initialize_password(password1.to_string(), password2.to_string());
            ui.global::<ConfigData>().set_password_match(result);
        }
     });

    // Change password button
     ui.global::<ConfigData>().on_change_password({
        let ui = ui_handle.unwrap();
        let data_path = config.borrow().get_data_path().clone();
        move |current_password, new_password| {
            let data_path_full:String  = format!("{}/{}",data_path,"secrets.db");
            config::change_password(data_path_full, current_password.to_string(), new_password.to_string());
            ui.global::<ConfigData>().set_change_password_status("Password changed successfully".into());
        }
     });     

     // view the seed phrase
     ui.global::<ConfigData>().on_view_seed_phrase({
        let ui = ui_handle.unwrap();
        let data_path = config.borrow().get_data_path().clone();
        move |password: SharedString| {
            let data_path_full:String  = format!("{}/{}",data_path,"secrets.db");
            let mut file = std::fs::File::open(data_path_full).unwrap();
            let correct = true;
            let secret_data: data::SecretData = data::SecretData::from_file(&mut file, password.as_str()).unwrap_or_else(
                |error| {
                    data::SecretData::from_mnemonic(BAD_MNEMONIC.to_string()).unwrap()
                }
            );
            let seed_phrase: String = secret_data.get_seed_phrase();
            if seed_phrase.clone() == BAD_MNEMONIC {
                ui.global::<ConfigData>().set_seed_phrase("Password is incorrect".into());
            } else {
                ui.global::<ConfigData>().set_seed_phrase(seed_phrase.into());
            }
        }
     });
 
    ui.run()?;

    Ok(())
}
