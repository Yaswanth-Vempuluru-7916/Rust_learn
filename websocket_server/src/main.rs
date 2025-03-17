use tokio::{net::TcpListener, sync::mpsc};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};


type ClientId = u32;
type ClientSink = tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>;
type ClientMap = Arc<Mutex<HashMap<ClientId, futures_util::stream::SplitSink<ClientSink, Message>>>>;
// Channel sender type for broadcasting messages
type Sender = mpsc::Sender<(ClientId,Message)>;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum AppMessage{
    Chat {sender : ClientId, text : String},
    Join {sender : ClientId},
    Leave {sender : ClientId}
}

#[tokio::main]
async fn main() -> Result<()> {
    
    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server started on ws://{}", addr);

    //shared State for all clients 
    let clients : ClientMap = Arc::new(Mutex::new(HashMap::new()));

    //Create a channel for broadcasting
    let (tx , mut rx) = mpsc::channel::<(ClientId,Message)>(100);

    //Spawn a broadcaster task
    let clients_clone = clients.clone();

    tokio::spawn(async move {
        while let Some((sender_id ,msg)) = rx.recv().await{
            let mut clients_guard = clients_clone.lock().await;

            for (client_id , ws_sink) in clients_guard.iter_mut(){
                // Locks the clients map, loops through all SplitSinks, and sends the message to everyone except the sender.
                if *client_id != sender_id {
                    if let Err(e) = ws_sink.send(msg.clone()).await {
                        println!("Error sending to client {}: {}", client_id, e);
                    }
                }
            }

        }
    });

    let mut client_id_counter = 0;

    while let Ok((stream ,_)) = listener.accept().await{
        let client_id = client_id_counter;
        client_id_counter+=1;
        let clients_clone = clients.clone();
        let tx_clone = tx.clone();

        // Send a Join message for the new client
        let join_msg = AppMessage::Join{sender : client_id};
        let json = serde_json::to_string(&join_msg)?;
        tx.send((client_id, Message::Text(json.into()))).await?;

        tokio::spawn(handle_connection(client_id ,stream , clients_clone, tx_clone));
    }
    Ok(())
}

async fn handle_connection(client_id : ClientId,stream : tokio::net::TcpStream,clients : ClientMap , tx : Sender) -> Result<()>{
   
    let  ws_stream = accept_async(stream).await?;
    println!("Client {}: WebSocket connection established", client_id);
    
    // Split and store the Sink
    let (ws_sink, mut ws_stream) = ws_stream.split();
    // Add client to the shared map
    {
        let mut clients_guard = clients.lock().await; 
        clients_guard.insert(client_id, ws_sink);
        println!("Client {} added. Total clients: {}", client_id, clients_guard.len());

    }

    // Handle incoming messages
    while let Some(msg) = ws_stream.next().await {
      /*   // ws_stream.next() returns an Option<Result<Message>>, so this unwraps the Result
        //  to get the actual Message (e.g., text or binary data).
        let msg = msg?;

        if msg.is_text() {
            let received_text = msg.to_text()?;
            println!("Client {} received message: {}", client_id, received_text);

            // let mut clients_guard = clients.lock().await;
            // if let Some(ws_sink) = clients_guard.get_mut(&client_id) {
            //     ws_sink.send(Message::Text(received_text.into())).await?;
            // }
            
            // Send to the broadcaster via the channel
            let broadcast_msg = Message::Text(received_text.into());
            if let Err(e) = tx.send((client_id,broadcast_msg)).await {
                println!("Failed to queue message from client {}: {}", client_id, e);
            }
            
        }
        */

        let msg = msg?;
        if let Message::Text(text) = msg {
            match serde_json::from_str::<AppMessage>(&text){
                Ok(app_msg)=>{
                    println!("Client {} sent: {:?}", client_id, app_msg);

                    match app_msg{
                        AppMessage::Chat { sender, text } =>{
                            let broadcast_msg = AppMessage::Chat { sender, text };
                            let json = serde_json::to_string(&broadcast_msg)?;
                            tx.send((client_id,Message::Text(json.into()))).await?;
                        }

                        AppMessage::Join { sender } =>{
                            println!("Client {} joined", sender);
                        }

                        AppMessage::Leave { sender } =>{
                            println!("Client {} requested to leave", sender);
                        }
                    }
                }

                Err(e) => println!("Client {} sent invalid JSON: {}", client_id, e),
            }
        }
    }

    let leave_msg = AppMessage::Leave { sender: client_id };
    let json = serde_json::to_string(&leave_msg)?;
    tx.send((client_id, Message::Text(json.into()))).await?;
    {
        let mut clients_guard = clients.lock().await;
        clients_guard.remove(&client_id);
        println!("Client {} disconnected. Total clients: {}", client_id, clients_guard.len());
        
    }
    Ok(())
}
