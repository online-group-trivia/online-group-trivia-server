use crate::data_model::{GameInfo, RoomInfo, TeamInfo};
use redis::Commands;
use serde::{Deserialize, Serialize};
use simple_error::SimpleError;
use std::error::Error;
use uuid::Uuid;
// TODO use connection pool

pub fn create_game(title: &String) -> Result<GameInfo, Box<dyn Error>> {
    let id = Uuid::new_v4();
    let client = redis::Client::open("redis://redis:6379")?;
    let mut con = client.get_connection()?;

    let game_info = GameInfo {
        id,
        title: title.to_owned(),
        questions: vec![],
    };

    let _: () = con.set(id.to_string(), serde_json::to_string(&game_info)?)?;
    Ok(game_info)
}

pub fn get_game_info(game_id: &Uuid) -> Result<GameInfo, Box<dyn Error>> {
    let client = redis::Client::open("redis://redis:6379")?;
    let mut con = client.get_connection()?;
    let game_info: String = con.get(game_id.to_string())?;
    let game_info: GameInfo = serde_json::from_str(&*game_info)?;
    Ok(game_info)
}

pub fn update_game(id: &Uuid, data: String) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://redis:6379")?;
    let mut con = client.get_connection()?;
    if !con.exists(id.to_string())? {
        Err(SimpleError::new("Game ID not found"))?
    }

    let _: () = con.set(id.to_string(), data)?;
    Ok(())
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize)]
pub struct RedisRoomInfo {
    pub title: String,
    pub teams_info: Vec<TeamInfo>,
    pub questions: Vec<String>,
}

impl RedisRoomInfo {
    fn new(room_info: &RoomInfo) -> RedisRoomInfo {
        RedisRoomInfo {
            title: room_info.title.to_owned(),
            teams_info: room_info.teams_info.to_owned(),
            questions: room_info.questions.to_owned(),
        }
    }
}

pub fn create_room(room_info: &RoomInfo) -> Result<GameInfo, Box<dyn Error>> {
    let client = redis::Client::open("redis://redis:6379")?;
    let mut con = client.get_connection()?;

    let redis_room_info = RedisRoomInfo::new(room_info);

    let _: () = con.set(redis_room_info.id, serde_json::to_string(&redis_room_info)?)?;
    Ok(game_info)
}
