use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::{Message, WebSocketConfig}};
use url::Url;

use super::request;

// Define an enum to represent the different modes
#[derive(Debug)]
pub enum Mode {
    Creative,
    Balanced,
    Precise,
}

// Define a struct to hold the data
#[derive(Debug)]
pub struct ConnectionInfo {
    pub sec_access_token: String,
    pub id: String,
    pub conversation_id: String,
    pub mode: Mode,
}

// The function to connect to the WebSocket.
pub async fn connect_to_websocket(data: request::ReturnData) -> Result<(), Box<dyn std::error::Error>> {
    // The WebSocket URL to which we want to connect.
    let sec_access_token = match data.x_sydney_conversationsignature {
        Some(token) => token,
        None => return Err("Missing x_sydney_conversationsignature header value".into()),
    };
    let ws_url = Url::parse(&format!("wss://sydney.bing.com/sydney/ChatHub?sec_access_token={}", sec_access_token))?;

    // Optional: Specify a custom WebSocket configuration here.
    //let config: Option<WebSocketConfig> = None; // or Some(WebSocketConfig { ... })

    // Connect to the WebSocket server.
    let (mut ws_stream, _) = connect_async(ws_url).await?;

    // The connection is established, and `ws_stream` is a WebSocket stream.

    // Send the '{"protocol":"json","version":1}' message
    let msg1 = r#"{"protocol":"json","version":1}"#;
    let msg1_with_separator = format!("{}", msg1);
    ws_stream.send(Message::Text(msg1_with_separator)).await?;

    // Send the '{"type":6}' message
    let msg2 = r#"{"type":6}"#;
    let msg2_with_separator = format!("{}", msg2);
    ws_stream.send(Message::Text(msg2_with_separator)).await?;

    // Listen for incoming messages
    while let Some(message) = ws_stream.next().await {
        match message? {
            Message::Text(text) => println!("Received text message: {}", text),
            Message::Binary(bin) => println!("Received binary message: {:?}", bin),
            _ => (), // Other message types can be handled here
        }
    }

    Ok(())
}