use serde::{Deserialize, Serialize};
use crate::data_model::{TeamInfo, RoomInfo};

#[derive(Serialize, Deserialize)]
pub struct RedisGameInfo {
    pub title: String,
    pub questions: Vec<String>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize)]
pub struct RedisRoomInfo {
    pub title: String,
    pub teams_info: Vec<TeamInfo>,
    pub questions: Vec<String>,
}

impl RedisRoomInfo {
    pub fn new(room_info: &RoomInfo) -> RedisRoomInfo {
        RedisRoomInfo {
            title: room_info.title.clone(),
            teams_info: room_info.teams_info.clone(),
            questions: room_info.questions.clone(),
        }
    }
}