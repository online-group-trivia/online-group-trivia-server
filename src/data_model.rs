use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Serialize, Deserialize, Clone)]
pub struct TeamInfo {
    pub name: String,
    pub participants: Vec<Participant>,
    pub score: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Participant {
    pub name: String,
}
