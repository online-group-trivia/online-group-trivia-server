use serde::{Deserialize};

#[derive(Deserialize)]
enum WebsocketMessage {
    Echo { body: String }
}

pub fn handle_client_message(text: &str) -> String {
    let websocket_message: WebsocketMessage = serde_json::from_str(text).unwrap();
    match websocket_message {
        WebsocketMessage::Echo { body } => body
    }
}