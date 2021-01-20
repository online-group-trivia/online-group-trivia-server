use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::database::data_model::RedisRoomInfo;

#[derive(Serialize, Deserialize, Clone)]
pub struct GameInfo {
    pub id: Uuid,
    pub title: String,
    pub questions: Vec<String>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone)]
pub struct RoomInfo {
    pub id: String,
    pub title: String,
    pub teams_info: Vec<TeamInfo>,
    pub questions: Vec<String>,
}

impl RoomInfo {
    pub fn new(id: &String, redis_room_info: RedisRoomInfo) -> RoomInfo {
        RoomInfo {
            id: id.to_owned(),
            title: redis_room_info.title,
            teams_info: redis_room_info.teams_info,
            questions: redis_room_info.questions
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TeamInfo {
    pub name: String,
    pub participants: Vec<Participant>,
    pub score: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Participant {
    pub name: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize)]
pub struct JoinRoomRequest {
    pub id: String,
    pub display_name: String,
}
