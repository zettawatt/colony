use serde::{Serialize, Deserialize};
use std::fs::{File, create_dir_all, read_to_string, write};
use std::path::PathBuf;
use dirs;
use toml;
//use toml_edit::{DocumentMut, value};

// Configuration file struct
#[derive(Serialize, Deserialize)]
pub struct Config {
    downloads_path: String,
    data_path: String,
    password_timeout: u64,
}

impl Config {
    fn new() -> Self {
        let data_path: PathBuf = dirs::data_dir().unwrap();
        let data_path: PathBuf = data_path.join("colony");
        let downloads_path: PathBuf = dirs::download_dir().unwrap();

        Config {
            downloads_path: downloads_path.to_string_lossy().into_owned(),
            data_path: data_path.to_string_lossy().into_owned(),
            password_timeout: 60,
        }
    }
}

fn get_config_path() -> PathBuf {
    let mut config_path: PathBuf = dirs::config_dir().expect("The configuration file path to your OS was not found");
    config_path.push("colony");
    config_path
}

fn get_config_file_path() -> PathBuf {
    let mut config_path = get_config_path();
    config_path.push("config");
    config_path.set_extension("toml");
    config_path
}

// Function to read the configuration file
// If the file does not exist, create it with default values
// If the file exists, read its contents
pub fn read_config() -> std::io::Result<Config> {

    // Build the OS independent path to the configuration file
    let config_path: PathBuf = get_config_file_path();

    // Open the file, but if it doesn't exist, create it
    let contents: String = read_to_string(&config_path)
        .unwrap_or_else( |error| -> String {
            if error.kind() == std::io::ErrorKind::NotFound {
                // Create the config file if it doesn't exist
                File::create(&config_path).unwrap_or_else(|error|{
                    if error.kind() == std::io::ErrorKind::NotFound {
                        create_dir_all(config_path.parent().unwrap()).unwrap();
                        File::create(&config_path).unwrap()
                    } else {
                        panic!("Problem creating the config file: {:?}", error);
                    }
                });
                // create a new config file with the default values
                let default_config_string: String = toml::to_string(&Config::new()).unwrap();
                let _ = write(&config_path,default_config_string.as_str());
                read_to_string(&config_path).unwrap()
            } else {
                panic!("Problem opening the config file: {:?}", error);
            }
        });

    // Parse the contents of the file
    let result: Config = toml::from_str(&contents).unwrap_or_else(|error| {
        panic!("Problem parsing the configuration information: {:?}", error);
    });
    
    println!("With text:\n{contents}");
    Ok(result)
}
