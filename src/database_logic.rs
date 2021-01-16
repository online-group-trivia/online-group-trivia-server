use serde::{Deserialize, Serialize};
use redis::Commands;
use std::error::Error;
use simple_error::SimpleError;
use uuid::Uuid;
// TODO use connection pool

#[derive(Serialize, Deserialize, Debug)]
pub struct GameInfo {
    id: Uuid,
    title: String,
    questions: Vec<String>,
}

pub fn create_game(title:&String) -> Result<GameInfo, Box<dyn Error>> {
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

pub fn get_game_info(game_id: String) -> Result<String, Box<dyn Error>> {
    let client = redis::Client::open("redis://redis:6379")?;
    let mut con = client.get_connection()?;
    let game_info = con.get(game_id)?;
    Ok(game_info)
}

pub fn update_game(id: String, data: String) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://redis:6379")?;
    let mut con = client.get_connection()?;
    if !con.exists(&id)? {
        Err(SimpleError::new("Game ID not found"))?
    }

    let _: () = con.set(&id, data)?;
    Ok(())
}
