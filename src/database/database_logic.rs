use crate::data_model::{GameInfo, RoomInfo, TeamInfo};
use crate::database::data_model::RedisGameInfo;
use redis::Commands;
use serde::{Deserialize, Serialize};
use simple_error::SimpleError;
use std::error::Error;
use uuid::Uuid;
// TODO use connection pool

const REDIS_ENDPOINT: &str = "redis://localhost:6379";

pub fn create_game(title: &String) -> Result<GameInfo, Box<dyn Error>> {
    let id = Uuid::new_v4();
    let client = redis::Client::open(REDIS_ENDPOINT)?;
    let mut con = client.get_connection()?;

    let game_info = GameInfo {
        id,
        title: title.to_owned(),
        questions: vec![],
    };

    let _: () = con.set(id.to_string(), serde_json::to_string(&game_info)?)?;
    Ok(game_info)
}

pub fn get_game_info(game_id: &Uuid) -> Result<RedisGameInfo, Box<dyn Error>> {
    let client = redis::Client::open(REDIS_ENDPOINT)?;
    let mut con = client.get_connection()?;
    let game_info: String = con.get(game_id.to_string())?;
    println!("{:?}", game_info);
    let redis_game_info: RedisGameInfo = serde_json::from_str(&*game_info)?;
    Ok(redis_game_info)
}

pub fn update_game(id: &Uuid, info: &RedisGameInfo) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open(REDIS_ENDPOINT)?;
    let mut con = client.get_connection()?;
    if !con.exists(id.to_string())? {
        Err(SimpleError::new("Game ID not found"))?
    }

    let _: () = con.set(id.to_string(), serde_json::to_string(info)?)?;
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
            title: room_info.title.clone(),
            teams_info: room_info.teams_info.clone(),
            questions: room_info.questions.clone(),
        }
    }
}

pub fn create_room(room_info: &RoomInfo) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open(REDIS_ENDPOINT)?;
    let mut con = client.get_connection()?;

    let redis_room_info = RedisRoomInfo::new(room_info);

    con.set(
        room_info.id.as_str(),
        serde_json::to_string(&redis_room_info)?,
    )?;
    Ok(())
}
