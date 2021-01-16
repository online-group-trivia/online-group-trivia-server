use crate::database_logic;
use crate::database_logic::GameInfo;
use std::error::Error;

pub fn create_room(room_name:&String) -> Result<GameInfo,Box<dyn Error>> {
    database_logic::create_game(room_name)
}