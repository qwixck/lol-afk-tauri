// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config_struct;
use std::io::Write;
use config_struct::Config;
use tauri::api::path::local_data_dir;

#[tauri::command]
fn write(name: String) -> bool {
    let is_in_config: bool;
    let file = std::fs::File::open(local_data_dir().unwrap().join("com.tauri.dev/data/config.json")).unwrap();
    let mut json: Config = serde_json::from_reader(file).unwrap();

    let index = json.pick.drafts.middle.iter().position(|x| x == &name);

    if let Some(iindex) = index {
        json.pick.drafts.middle.remove(iindex);
        is_in_config = true;
    } else {
        json.pick.drafts.middle.push(name);
        is_in_config = false;
    }

    let updated_json = serde_json::to_string_pretty(&json)
        .expect("Failed to serialize to JSON");
    
    let mut file = std::fs::File::create(local_data_dir().unwrap().join("com.tauri.dev/data/config.json"))
        .expect("Failed to create file");
    
    file.write_all(updated_json.as_bytes())
        .expect("Couldn't write to file");

    return is_in_config;
}

#[tauri::command]
fn read(name: String) -> bool {
    let file = std::fs::File::open(local_data_dir().unwrap().join("com.tauri.dev/data/config.json")).unwrap();
    let json: Config = serde_json::from_reader(file).unwrap();
    let index = json.pick.drafts.middle.iter().position(|x| x == &name);

    return index.is_some();
}

#[tauri::command]
fn get_champions() -> String{
    let file = std::fs::File::open(local_data_dir().unwrap().join("com.tauri.dev/data/champions.json")).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    
    return json.to_string();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![write, read, get_champions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}