// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod structs;
use std::io::Write;
use structs::{Settings, Config};
use serde_json;
use tauri::api::path::local_data_dir;

#[tauri::command]
fn write(name: String) -> bool {
    let is_in_config: bool;
    let file = std::fs::File::open(local_data_dir().unwrap().join("com.tauri.dev/data/config.json")).unwrap();
    let mut config: Config = serde_json::from_reader(file).unwrap();
    let file = std::fs::File::open(local_data_dir().unwrap().join("com.tauri.dev/data/settings.json")).unwrap();
    let settings: Settings = serde_json::from_reader(file).unwrap();

    match settings.type_.as_str() {
        "pick" => match settings.mode.as_str() {
            "drafts" => match settings.position.as_str() {
                "top" => {
                    if let Some(index) = config.pick.drafts.top.iter().position(|x| x == &name) {
                        config.pick.drafts.top.remove(index);
                        is_in_config = true;
                    } else {
                        config.pick.drafts.top.push(name);
                        is_in_config = false;
                    }
                },
                "jungle" => {
                    if let Some(index) = config.pick.drafts.jungle.iter().position(|x| x == &name) {
                        config.pick.drafts.jungle.remove(index);
                        is_in_config = true;
                    } else {
                        config.pick.drafts.jungle.push(name);
                        is_in_config = false;
                    }
                }
                "middle" => {
                    if let Some(index) = config.pick.drafts.jungle.iter().position(|x| x == &name) {
                        config.pick.drafts.middle.remove(index);
                        is_in_config = true;
                    } else {
                        config.pick.drafts.middle.push(name);
                        is_in_config = false;
                    }
                }
                "bottom" => {
                    if let Some(index) = config.pick.drafts.bottom.iter().position(|x| x == &name) {
                        config.pick.drafts.bottom.remove(index);
                        is_in_config = true;
                    } else {
                        config.pick.drafts.bottom.push(name);
                        is_in_config = false;
                    }
                }
                "utility" => {
                    if let Some(index) = config.pick.drafts.utility.iter().position(|x| x == &name) {
                        config.pick.drafts.utility.remove(index);
                        is_in_config = true;
                    } else {
                        config.pick.drafts.utility.push(name);
                        is_in_config = false;
                    }
                }
                &_ => todo!()
            },
            "blind" => {
                if let Some(index) = config.pick.blind.middle.iter().position(|x| x == &name) {
                    config.pick.blind.middle.remove(index);
                    is_in_config = true;
                } else {
                    config.pick.blind.middle.push(name);
                    is_in_config = false;
                }
            }
            &_ => todo!()
        },
        "ban" => match settings.mode.as_str() {
            "drafts" => match settings.position.as_str() {
                "top" => {
                    if let Some(index) = config.ban.drafts.top.iter().position(|x| x == &name) {
                        config.ban.drafts.top.remove(index);
                        is_in_config = true;
                    } else {
                        config.pick.drafts.top.push(name);
                        is_in_config = false;
                    }
                },
                "jungle" => {
                    if let Some(index) = config.ban.drafts.jungle.iter().position(|x| x == &name) {
                        config.ban.drafts.jungle.remove(index);
                        is_in_config = true;
                    } else {
                        config.pick.drafts.jungle.push(name);
                        is_in_config = false;
                    }
                }
                "middle" => {
                    if let Some(index) = config.ban.drafts.jungle.iter().position(|x| x == &name) {
                        config.ban.drafts.middle.remove(index);
                        is_in_config = true;
                    } else {
                        config.pick.drafts.middle.push(name);
                        is_in_config = false;
                    }
                }
                "bottom" => {
                    if let Some(index) = config.ban.drafts.bottom.iter().position(|x| x == &name) {
                        config.ban.drafts.bottom.remove(index);
                        is_in_config = true;
                    } else {
                        config.pick.drafts.bottom.push(name);
                        is_in_config = false;
                    }
                }
                "utility" => {
                    if let Some(index) = config.ban.drafts.utility.iter().position(|x| x == &name) {
                        config.ban.drafts.utility.remove(index);
                        is_in_config = true;
                    } else {
                        config.pick.drafts.utility.push(name);
                        is_in_config = false;
                    }
                }
                &_ => todo!()
            },
            &_ => todo!()
        },
        &_ => todo!()
    };

    let updated_json = serde_json::to_string_pretty(&config)
        .expect("Failed to serialize to JSON");
    
    let mut file = std::fs::File::create(local_data_dir().unwrap().join("com.tauri.dev/data/config.json"))
        .expect("Failed to create file");
    
    file.write_all(updated_json.as_bytes())
        .expect("Couldn't write to file");

    return is_in_config;
}

#[tauri::command]
fn read(name: String) -> bool {
    let file = std::fs::File::open(local_data_dir().unwrap().join("com.tauri.dev/data/settings.json")).unwrap();
    let settings: Settings = serde_json::from_reader(file).unwrap();
    let file = std::fs::File::open(local_data_dir().unwrap().join("com.tauri.dev/data/config.json")).unwrap();
    let config: Config = serde_json::from_reader(file).unwrap();

    // TODO: fix crashing when pressing blind button
    let something = match settings.type_.as_str() {
        "pick" => match settings.mode.as_str() {
            "drafts" => match settings.position.as_str() {
                "top" => config.pick.drafts.top.iter().position(|x| x == &name),
                "jungle" => config.pick.drafts.jungle.iter().position(|x| x == &name),
                "middle" => config.pick.drafts.middle.iter().position(|x| x == &name),
                "bottom" => config.pick.drafts.bottom.iter().position(|x| x == &name),
                "utility" => config.pick.drafts.utility.iter().position(|x| x == &name),
                &_ => todo!()
            },
            "blind" => config.pick.blind.middle.iter().position(|x| x == &name),
            &_ => todo!()
        },
        "ban" => match settings.mode.as_str() {
            "drafts" => match settings.position.as_str() {
                "top" => config.pick.drafts.top.iter().position(|x| x == &name),
                "jungle" => config.ban.drafts.jungle.iter().position(|x| x == &name),
                "middle" => config.ban.drafts.middle.iter().position(|x| x == &name),
                "bottom" => config.ban.drafts.bottom.iter().position(|x| x == &name),
                "utility" => config.ban.drafts.utility.iter().position(|x| x == &name),
                &_ => todo!()
            },
            &_ => todo!()
        },
        &_ => todo!()
    };

    return something.is_some()
    
}

#[tauri::command]
fn get_champions() -> String{
    let file = std::fs::File::open(local_data_dir().unwrap().join("com.tauri.dev/data/champions.json")).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    
    return json.to_string();
}

#[tauri::command]
fn get_setting(setting: String) -> serde_json::Value {
    let file = std::fs::File::open(local_data_dir().unwrap().join("com.tauri.dev/data/settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();

    return json[setting].clone();
}

#[tauri::command]
fn change_setting(key: String, value: String) {
    let file = std::fs::File::open(local_data_dir().unwrap().join("com.tauri.dev/data/settings.json"))
        .expect("Couldn't open file");

    let mut json: serde_json::Value = serde_json::from_reader(file).unwrap();

    json[key] = serde_json::Value::String(value);

    let update_json = serde_json::to_string_pretty(&json)
        .expect("Failed to serialize to JSON");

    let mut file = std::fs::File::create(local_data_dir().unwrap().join("com.tauri.dev/data/settings.json"))
        .expect("Failed to create file");

    file.write_all(update_json.as_bytes())
        .expect("Couldn't write to file");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read, write, get_champions, change_setting, get_setting])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}