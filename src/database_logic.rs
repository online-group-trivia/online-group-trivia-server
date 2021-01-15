use serde::{Deserialize, Serialize};
use redis::Commands;
use std::error::Error;
use simple_error::SimpleError;

// TODO use connection pool

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomInfo {
    title: String,
    questions: Vec<String>,
}

pub fn create_room(room_uuid: String, room_name:&String) -> Result<(), Box<dyn Error>> {
    // TODO Return room info as well
    // TODO Rename (room/game/template)
    let client = redis::Client::open("redis://redis:6379")?;
    let mut con = client.get_connection()?;
    let room_info = RoomInfo {
        title: room_name.to_owned(),
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

pub fn update_room(room_uuid: String, data: String) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://redis:6379")?;
    let mut con = client.get_connection()?;
    if !con.exists(&room_uuid)? {
        Err(SimpleError::new("Room ID not found"))?
    }

    let _: () = con.set(&room_uuid, data)?;
    Ok(())
}