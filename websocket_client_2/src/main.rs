use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use anyhow::Result;
use futures_util:: StreamExt;
use url::Url;
use serde::{Serialize, Deserialize};

type ClientId = u32;

#[derive(Serialize, Deserialize, Debug)]
enum AppMessage {
    Chat { sender: ClientId, text: String },
    Join { sender: ClientId },
    Leave { sender: ClientId },
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = Url::parse("ws://127.0.0.1:8080")?;
    let (mut ws_stream, _) = connect_async(url.as_str()).await?;
    println!("WebSocket client 2 connected");

    // Receive messages only (no sending)
    while let Some(msg) = ws_stream.next().await {
        match msg? {
            Message::Text(text) => {
                match serde_json::from_str::<AppMessage>(&text) {
                    Ok(app_msg) => {
                        match app_msg {
                            AppMessage::Chat { sender, text } => {
                                println!("Chat from {}: {}", sender, text);
                            }
                            AppMessage::Join { sender } => {
                                println!("Client {} joined", sender);
                            }
                            AppMessage::Leave { sender } => {
                                println!("Client {} left", sender);
                            }
                        }
                    }
                    Err(e) => println!("Received invalid JSON: {}", e),
                }
            }
            _ => {}
        }
    }

    Ok(())
}