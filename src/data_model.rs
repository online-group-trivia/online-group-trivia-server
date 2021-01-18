use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct GameInfo {
    pub id: Uuid,
    pub title: String,
    pub questions: Vec<String>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize)]
pub struct RoomInfo {
    pub id: String,
    pub title: String,
    pub teams_info: Vec<TeamInfo>,
    pub questions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TeamInfo {
    pub name: String,
    pub participants: Vec<Participant>,
    pub score: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Participant {
    pub name: String,
}
