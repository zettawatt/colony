use serde::{Serialize, Deserialize};
use std::fs::{File, create_dir_all, read_to_string, write};
use std::path::PathBuf;
use bip39::{Mnemonic, Language};
use dirs;
use toml;
//use toml_edit::{DocumentMut, value};

// Configuration file struct
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub downloads_path: String,
    pub data_path: String,
    pub password_timeout: u64,
}

impl Config {
    fn new() -> Self {
        let mut data_path: PathBuf = dirs::data_dir().expect("the data directory path to your OS was not found");
        //let data_path: PathBuf = data_path.join("colony");
        data_path.push("colony");
        let downloads_path: PathBuf = dirs::download_dir().unwrap_or(data_path.clone());

        Config {
            downloads_path: downloads_path.to_string_lossy().into_owned(),
            data_path: data_path.to_string_lossy().into_owned(),
            password_timeout: 60,
        }
    }

    pub fn set_config(&mut self, downloads_path: String, data_path: String, password_timeout: u64) {
        self.downloads_path = downloads_path;
        self.data_path = data_path;
        self.password_timeout = password_timeout;
        write_config(self).expect("Problem writing the config file");
    }
}

// Seed phrase struct
pub struct SeedPhrase {
    pub seed_words: Vec<String>,
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

fn write_config(config: &Config) -> std::io::Result<()> {
    let config_string: String = toml::to_string(config).unwrap();
    let config_path: PathBuf = get_config_file_path();
    write(&config_path,config_string.as_str())?;
    Ok(())
}

// Function to read the configuration file
// If the file does not exist, create it with default values
// If the file exists, read its contents
pub fn read_config() -> (Config, bool) {

    // Build the OS independent path to the configuration file
    let config_path: PathBuf = get_config_file_path();

    // Open the file, but if it doesn't exist, create it
    let mut initialized: bool = true;
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
                initialized = false;
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
    
    //println!("With text:\n{contents}");
    (result, initialized)
}

pub fn initialize_password(password1: String, password2: String) -> bool {
    let result: bool = password1 == password2;
    //FIXME: need to pass a handle to this newly created password to the UI for the key generation step
    result
}

pub fn generate_seed_phrase() -> SeedPhrase {
    let m = Mnemonic::generate_in(Language::English, 12).unwrap();
    let seed_phrase = m.words();
    let seed_words: Vec<String> = seed_phrase.map(|s| s.to_string()).collect();
    SeedPhrase {
        seed_words,
    }
}
