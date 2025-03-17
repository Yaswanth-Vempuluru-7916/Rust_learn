use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

type ClientId = u32;
type ClientSink = tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>;
type ClientMap = Arc<Mutex<HashMap<ClientId, futures_util::stream::SplitSink<ClientSink, Message>>>>;
#[tokio::main]
async fn main() -> Result<()> {
    
    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server started on ws://{}", addr);

    //shared State for all clients 
    let clients : ClientMap = Arc::new(Mutex::new(HashMap::new()));
    let mut client_id_counter = 0;

    while let Ok((stream ,_)) = listener.accept().await{
        let client_id = client_id_counter;
        client_id_counter+=1;
        let clients_clone = clients.clone();
        tokio::spawn(handle_connection(client_id ,stream , clients_clone));
    }
    Ok(())
}

async fn handle_connection(client_id : ClientId,stream : tokio::net::TcpStream,clients : ClientMap) -> Result<()>{
    let  ws_stream = accept_async(stream).await?;
    println!("Client {}: WebSocket connection established", client_id);

    let (ws_sink, mut ws_stream) = ws_stream.split();

    // Add client to the shared map

    {
        let mut clients_guard = clients.lock().await; 
        clients_guard.insert(client_id, ws_sink);
        println!("Client {} added. Total clients: {}", client_id, clients_guard.len());

    }

    while let Some(msg) = ws_stream.next().await {
        // ws_stream.next() returns an Option<Result<Message>>, so this unwraps the Result
        //  to get the actual Message (e.g., text or binary data).
        let msg = msg?;

        if msg.is_text() {
            let received_text = msg.to_text()?;
            println!("Client {} received message: {}", client_id, received_text);

            let mut clients_guard = clients.lock().await;
            if let Some(ws_sink) = clients_guard.get_mut(&client_id) {
                ws_sink.send(Message::Text(received_text.into())).await?;
            }
            
            
        }
    }
    {
        let mut clients_guard = clients.lock().await;
        clients_guard.remove(&client_id);
        println!("Client {} disconnected. Total clients: {}", client_id, clients_guard.len());
        
    }
    Ok(())
}
