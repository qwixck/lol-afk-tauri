use futures_util::stream::StreamExt;
use serde_json::json;

#[tauri::command]
pub async fn connect() -> Result<(), String> {
    if let Err(error) = lcu_connect().await {
        eprintln!("Error connecting: {:?}", error);
        return Err(error.to_string());
    }

    Ok(())
}

async fn lcu_connect() -> Result<(), shaco::error::LcuWebsocketError> {
    let rest = shaco::rest::RESTClient::new().unwrap();
    let mut client = shaco::ws::LcuWebsocketClient::connect().await?;
    client
        .subscribe(shaco::model::ws::LcuSubscriptionType::JsonApiEvent("/lol-champ-select/v1/session".to_string()))
        .await
        .unwrap();

    while let Some(event) = client.next().await {
        let local_player_cell_id = &event.data["localPlayerCellId"];

        let lobby_phase = event.data["timer"]["phase"].as_str().unwrap().to_string();
        let mut position: String = "".to_string();
        let mut mode: String = "".to_string();
        let mut phase: String = "".to_string();
        let mut action_id: u64 = 0;
        let mut am_i_banning: bool = false;
        let mut am_i_picking: bool = false;

        for teammate in event.data["myTeam"].as_array().unwrap() {
            if teammate["cellId"].as_u64().unwrap() == local_player_cell_id.as_u64().unwrap() {
                position = teammate["assignedPosition"].as_str().unwrap().to_string();
                mode = "drafts".to_string();
            }
        }

        for action in event.data["actions"].as_array().unwrap() {
            for action_arr in action.as_array().unwrap() {
                if action_arr["actorCellId"].as_u64().unwrap() == local_player_cell_id.as_u64().unwrap() && action_arr["isInProgress"].as_bool().unwrap() {
                    phase = action_arr["type"].as_str().unwrap().to_string();
                    action_id = action_arr["id"].as_u64().unwrap();
                    
                    if phase == "ban".to_string() {
                        am_i_banning = action_arr["isInProgress"].as_bool().unwrap();
                    }
                    if phase == "pick".to_string() {
                        am_i_picking = action_arr["isInProgress"].as_bool().unwrap();
                    }
                }
            }
        }

        if rest
            .get("/lol-champ-select/v1/current-champion".to_string())
            .await
            .unwrap() != 0 {
                am_i_picking = false;  
            }

        if phase == "ban".to_string() && lobby_phase == "BAN_PICK".to_string() && am_i_banning {
            todo!();
        }

        if phase == "pick".to_string() && lobby_phase == "BAN_PICK".to_string() && am_i_picking {
            if let Err(_) = rest
                .patch(format!("/lol-champ-select/v1/session/actions/{}", action_id).to_string(), json!({
                    "championId": 39,
                    "completed": true
                }))
                .await {
                    am_i_picking = false;
            }
        }
    }

    Ok(())

}