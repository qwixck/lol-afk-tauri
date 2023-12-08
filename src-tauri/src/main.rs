// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lol;

use std::io::Write;
use tauri::api::path::local_data_dir;

#[tauri::command]
fn write(name: String) -> bool {
    let is_in_config: bool;
    let file = std::fs::File::open(local_data_dir().unwrap().join("lol-afk/data/config.json")).unwrap();
    let mut config: serde_json::Value = serde_json::from_reader(file).unwrap();
    let file = std::fs::File::open(local_data_dir().unwrap().join("lol-afk/data/settings.json")).unwrap();
    let settings: serde_json::Value = serde_json::from_reader(file).unwrap();

    if let Some(index) = config
        [settings["type_"].as_str().unwrap()]
        [settings["mode"].as_str().unwrap()]
        [settings["position"].as_str().unwrap()]
        .as_array().unwrap().iter().position(|x| x == &name) {
            config
                [settings["type_"].as_str().unwrap()]
                [settings["mode"].as_str().unwrap()]
                [settings["position"].as_str().unwrap()]
                .as_array_mut().unwrap().remove(index);

            is_in_config = true;
    } else {
        config
            [settings["type_"].as_str().unwrap()]
            [settings["mode"].as_str().unwrap()]
            [settings["position"].as_str().unwrap()]
            .as_array_mut().unwrap().push(serde_json::Value::String(name));
        
        is_in_config = false;
    }

    let updated_json = serde_json::to_string_pretty(&config).unwrap();
    
    let mut file = std::fs::File::create(local_data_dir().unwrap().join("lol-afk/data/config.json")).unwrap();
    
    file.write_all(updated_json.as_bytes()).unwrap();

    return is_in_config;
}

#[tauri::command]
fn read(name: String) -> bool {
    let file = std::fs::File::open(local_data_dir().unwrap().join("lol-afk/data/settings.json")).unwrap();
    let settings: serde_json::Value = serde_json::from_reader(file).unwrap();
    let file = std::fs::File::open(local_data_dir().unwrap().join("lol-afk/data/config.json")).unwrap();
    let config: serde_json::Value = serde_json::from_reader(file).unwrap();

    let something = config
        [settings["type_"].as_str().unwrap()]
        [settings["mode"].as_str().unwrap()]
        [settings["position"].as_str().unwrap()]
        .as_array().unwrap().iter().position(|x| x == &name);
    
    return something.is_some()
}

#[tauri::command]
fn get_champions() -> String {
    let file = std::fs::File::open("./data/champions.json").unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    
    return json.to_string();
}

#[tauri::command]
fn get_setting(setting: String) -> serde_json::Value {
    let file = std::fs::File::open(local_data_dir().unwrap().join("lol-afk/data/settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();

    return json[setting].clone();
}

#[tauri::command]
fn change_setting(key: String, value: String) {
    let file = std::fs::File::open(local_data_dir().unwrap().join("lol-afk/data/settings.json")).unwrap();

    let mut json: serde_json::Value = serde_json::from_reader(file).unwrap();

    json[key] = serde_json::Value::String(value);

    let update_json = serde_json::to_string_pretty(&json).unwrap();

    let mut file = std::fs::File::create(local_data_dir().unwrap().join("lol-afk/data/settings.json")).unwrap();

    file.write_all(update_json.as_bytes()).unwrap();
}

fn main() {
    if !local_data_dir().unwrap().join("lol-afk").exists() {
        std::fs::create_dir_all(local_data_dir().unwrap().join("lol-afk/data")).unwrap();

        let file = std::fs::File::create(local_data_dir().unwrap().join("lol-afk/data/config.json")).unwrap();
        let data = serde_json::json!({
            "pick": {"drafts": {"top": [],"jungle": [],"middle": [],"bottom": [],"utility": []},"blind": {"middle": []}},
            "ban": {"drafts": {"top": [],"jungle": [],"middle": [],"bottom": [],"utility": []}}
        });
        serde_json::to_writer_pretty(file, &data).unwrap();

        let file = std::fs::File::create(local_data_dir().unwrap().join("lol-afk/data/settings.json")).unwrap();
        let data = serde_json::json!({
            "mode": "drafts", "position": "middle", "type_": "pick"
        });
        serde_json::to_writer_pretty(file, &data).unwrap();
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read, write, get_champions, change_setting, get_setting, lol::connect, lol::get_available_champions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}