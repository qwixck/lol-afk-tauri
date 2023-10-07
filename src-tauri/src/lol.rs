use futures_util::StreamExt;
use shaco::model::ingame::GameEvent::*;

#[tauri::command]
pub async fn connect() {
    let ingame_client = shaco::ingame::IngameClient::new().unwrap();
    let mut event_stream = shaco::ingame::EventStream::from_ingame_client(ingame_client, None);

    while let Some(game_event) = event_stream.next().await {
        match game_event {
            GameStart(event) => {
                println!("Game started! Time: {}", event.event_time);
            }
            TurretKilled(event) => {
                println!("Turred destyoyed! Killer: {}, time: {}", event.killer_name, event.event_time);
            }
            _ => {
                println!("Unknowm event. {:?}", game_event);
            }
        }
    }
}