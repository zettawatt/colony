// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//use colony::config::generate_seed_phrase;
use serde::Serialize;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_new_seed_phrase() -> Result<String, String> {
    //Ok(generate_seed_phrase())
    let seed_phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_string();
    Ok(seed_phrase)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_new_seed_phrase])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
