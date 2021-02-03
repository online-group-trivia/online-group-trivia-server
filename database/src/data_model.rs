use async_trait::async_trait;
use interfaces::{GameInfo, RoomInfo, UpdateGameCommand, UpdateRoomCommand};
use std::error::Error;
use std::fmt;
use uuid::Uuid;

#[derive(Debug)]
pub struct MyError(pub String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}

pub struct MongoDb {}

#[async_trait]
pub trait GroupTriviaDatabase {
    async fn create_game(&self, title: &String) -> Result<GameInfo, Box<dyn Error>>;
    async fn get_game_info(&self, game_id: &Uuid) -> Result<GameInfo, Box<dyn Error>>;
    async fn update_game(
        &self,
        id: &Uuid,
        command: &UpdateGameCommand,
    ) -> Result<(), Box<dyn Error>>;
    async fn create_room(&self, room_info: &RoomInfo) -> Result<(), Box<dyn Error>>;
    async fn get_room_info(&self, id: &String) -> Result<RoomInfo, Box<dyn Error>>;
    async fn update_room(
        &self,
        id: &String,
        command: UpdateRoomCommand,
    ) -> Result<RoomInfo, Box<dyn Error>>;
}
