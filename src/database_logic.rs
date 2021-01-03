use redis::RedisError;
use serde::{Deserialize, Serialize};
use redis::Commands;
use std::io;
use std::error::Error;


#[derive(Serialize, Deserialize, Debug)]
pub struct RoomInfo {
    title: String,
    questions: Vec<String>,
}

pub fn create_room(room_uuid: String) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://redis:6379")?;
    let mut con = client.get_connection()?;
    let room_info = RoomInfo {
        title: "Title".to_owned(),
        questions: vec![],
    };
    let _: () = con.set(room_uuid, serde_json::to_string(&room_info)?)?;
    Ok(())
}

pub fn get_room_info(room_uuid: String) -> Result<String, Box<dyn Error>> {
    let client = redis::Client::open("redis://redis:6379")?;
    let mut con = client.get_connection()?;
    let room_info = con.get(room_uuid)?;
    Ok(room_info)
}