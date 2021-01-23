use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
enum WebsocketMessage {
    Echo { body: String },
    ParticipantSubscription { room_id: String },
    ManagerSubscription { game_id: Uuid, room_id: String },
}

pub fn handle_client_message(text: &str) -> String {
    let websocket_message: WebsocketMessage = serde_json::from_str(text).unwrap();
    match websocket_message {
        WebsocketMessage::Echo { body } => body,
        WebsocketMessage::ParticipantSubscription { .. } => {}
        WebsocketMessage::ManagerSubscription { game_id, room_id } => {
            pub_sub::subscribe_to_topic(game_id);
            pub_sub::subscribe_to_topic(room_id)
        }
    }
}
