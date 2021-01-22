use crate::data_model::{RoomInfo, TeamInfo};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

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

pub enum UpdateGameCommand {
    AddQuestion(String),
    RemoveQuestion(String),
    ChangeTitle(String),
}

#[derive(Debug)]
pub struct MyError(pub String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}
