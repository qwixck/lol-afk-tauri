use futures_util::stream::StreamExt;
use serde_json::{self, json};
use tauri::api::path::local_data_dir;

use shaco::model::ws::LcuSubscriptionType::JsonApiEvent;

#[tauri::command]
pub async fn connect() -> Result<(), String> {
    if let Err(error) = lcu_connect().await {
        eprintln!("Error connecting: {:?}", error);
        return Err(error.to_string());
    }

    Ok(())
}

async fn lcu_connect() -> Result<(), shaco::error::LcuWebsocketError> {
    let champions: serde_json::Value = serde_json::from_reader(std::fs::File::open("./data/champions.json").unwrap()).unwrap();
    let config: serde_json::Value = serde_json::from_reader(std::fs::File::open(local_data_dir().unwrap().join("lol-afk/data/config.json")).unwrap()).unwrap();

    let mut client = shaco::ws::LcuWebsocketClient::connect().await?;
    let rest = shaco::rest::RESTClient::new().unwrap();
    
    client
        .subscribe(JsonApiEvent("/lol-champ-select/v1/session".to_string()))
        .await
        .unwrap();

    client
        .subscribe(JsonApiEvent("/lol-matchmaking/v1/ready-check".to_string()))
        .await
        .unwrap();

    let mut position: String = String::new();
    let mut mode: String = String::new();
    let mut phase: String = String::new();
    let mut action_id: i64 = 0;
    let mut am_i_banning: bool = false;
    let mut am_i_picking: bool = false;

    while let Some(event) = client.next().await {
        match event.subscription_type.to_string().as_str() {
            "lol-matchmaking_v1_ready-check" => {
                if event.data["state"].as_str().unwrap() == "InProgress" && event.data["playerResponse"].as_str().unwrap() == "None" {
                    rest
                        .post("/lol-matchmaking/v1/ready-check/accept".to_string(), json!({}))
                        .await
                        .unwrap();
                }
            }

            _ => {
                let local_player_cell_id = &event.data["localPlayerCellId"];
                let lobby_phase = event.data["timer"]["phase"].as_str().unwrap().to_string();

                for teammate in event.data["myTeam"].as_array().unwrap() {
                    if teammate["cellId"].as_i64().unwrap() == local_player_cell_id.as_i64().unwrap() {
                        position = teammate["assignedPosition"].as_str().unwrap().to_string();
                        mode = "drafts".to_string();
                    }
                }

                for action in event.data["actions"].as_array().unwrap() {
                    for action_arr in action.as_array().unwrap() {
                        if action_arr["actorCellId"].as_i64().unwrap() == local_player_cell_id.as_i64().unwrap() && action_arr["isInProgress"].as_bool().unwrap() {
                            phase = action_arr["type"].as_str().unwrap().to_string();
                            action_id = action_arr["id"].as_i64().unwrap();
                            
                            if phase == "ban".to_string() {
                                am_i_banning = action_arr["isInProgress"].as_bool().unwrap();
                            }
                            if phase == "pick".to_string() {
                                am_i_picking = action_arr["isInProgress"].as_bool().unwrap();
                            }
                        }
                    }
                }

                if phase == "ban".to_string() && lobby_phase == "BAN_PICK".to_string() && am_i_banning {
                    if config["ban"][mode.as_str()][position.as_str()].as_array().unwrap().iter().len() != 0 {
                        for i in config["ban"][mode.as_str()][position.as_str()].as_array().unwrap() {
                            if let Ok(_) = rest
                                .patch(format!("/lol-champ-select/v1/session/actions/{}", action_id).to_string(), json!({
                                    "championId": champions["data"][i.as_str().unwrap()]["key"].as_str().unwrap().parse::<i64>().unwrap(),
                                    "completed": true
                                }))
                                .await {
                                    am_i_banning = false;
                                    break;
                            }
                        }
                    }
                }

                if phase == "pick".to_string() && lobby_phase == "BAN_PICK".to_string() && am_i_picking {
                    if position == "".to_string() {
                        position = "middle".to_string();
                        mode = "blind".to_string();
                    }

                    if config["pick"][mode.as_str()][position.as_str()].as_array().unwrap().iter().len() != 0 {
                        for i in config["pick"][mode.as_str()][position.as_str()].as_array().unwrap() {
                            if let Ok(_) = rest
                                .patch(format!("/lol-champ-select/v1/session/actions/{}", action_id).to_string(), json!({
                                    "championId": champions["data"][i.as_str().unwrap()]["key"].as_str().unwrap().parse::<i64>().unwrap(),
                                    "completed": true
                                }))
                                .await {
                                    am_i_picking = false;
                                    break;
                            }

                            if rest
                                .get("/lol-champ-select/v1/current-champion".to_string())
                                .await
                                .unwrap().as_i64().unwrap() != 0 {
                                    am_i_picking = false;  
                                    break;
                            }
                        }
                    } else {
                        println!("No champion to select");
                        am_i_picking = false;
                    }
                }
            }
        }
    }

    Ok(())

}

#[tauri::command]
pub async fn get_available_champions() -> String {
    let rest = shaco::rest::RESTClient::new().unwrap();

    return rest
        .get("/lol-champions/v1/owned-champions-minimal".to_string())
        .await
        .unwrap()
        .to_string();
}