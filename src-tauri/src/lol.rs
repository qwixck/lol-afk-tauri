use futures_util::stream::StreamExt;

#[tauri::command]
pub async fn connect() -> Result<(), String> {
    if let Err(error) = lcu_connect().await {
        eprintln!("Error connecting: {:?}", error);
        return Err(error.to_string());
    }

    Ok(())
}

async fn lcu_connect() -> Result<(), shaco::error::LcuWebsocketError> {
    let mut client = shaco::ws::LcuWebsocketClient::connect().await?;
    client
        .subscribe(shaco::model::ws::LcuSubscriptionType::JsonApiEvent("/lol-champ-select/v1/session".to_string()))
        .await
        .unwrap();

    while let Some(event) = client.next().await {
        println!("Event: {:?}", event);
    }

    Ok(())

}
