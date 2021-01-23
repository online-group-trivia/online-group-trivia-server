use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameInfo {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub title: String,
    pub questions: Vec<String>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoomInfo {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub teams_info: Vec<TeamInfo>,
    pub questions: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TeamInfo {
    pub name: String,
    pub participants: Vec<Participant>,
    pub score: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Participant {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub enum UpdateGameCommand {
    AddQuestion { question: String },
    RemoveQuestion { question: String },
    ChangeTitle { title: String },
}
