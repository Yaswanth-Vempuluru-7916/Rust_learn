use serde::{Deserialize, Serialize};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use url::Url;

type ClientId = u32;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum AppMessage{
    Chat {sender : ClientId, text : String},
    Join {sender : ClientId},
    Leave {sender : ClientId}
}


#[tokio::main]
async fn main() -> Result<()> {
    let url = Url::parse("ws://127.0.0.1:8080")?;

    let (mut ws_stream , _) = connect_async(url.as_str()).await.expect("failed to connect");
    println!("WebSocket client connected"); 

    let client_id = 0;
    let msg = AppMessage::Chat { sender: client_id, text: "Hello Everyone   !".to_string() };
    let json = serde_json::to_string(&msg)?;

    ws_stream.send(Message::Text(json.into())).await?;

    while let Some(msg) = ws_stream.next().await {
        match msg? {
            // ws_stream.next() returns an Option<Result<Message>>
            // so ? extracts the Message or passes up errors.

            Message::Text(text)=>{
                match serde_json::from_str::<AppMessage>(&text){
                    Ok(app_msg)=>{
                        match app_msg{
                            AppMessage::Chat { sender, text }=>{
                                println!("Chat from {}: {}", sender, text);
                            }
                            AppMessage::Join { sender }=>{
                                println!("Client {} joined", sender);
                            }
                            AppMessage::Leave { sender } =>{
                                println!("Client {} left", sender);
                            }
                        }
                    }

                    Err(e)=>println!("Received invalid JSON: {}", e),
                }
            }
            _ => {}
        }
    }
    Ok(())

}
