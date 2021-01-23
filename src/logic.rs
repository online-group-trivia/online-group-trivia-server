use crate::data_model::{GameInfo, RoomInfo, TeamInfo};
use crate::database::data_model::UpdateGameCommand;
use crate::database::mongo_db;
use crate::Uuid;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::error::Error;

pub async fn create_game(title: &String) -> Result<GameInfo, Box<dyn Error>> {
    mongo_db::create_game(title).await
}

pub async fn update_game(id: &Uuid, command: &UpdateGameCommand) -> Result<(), Box<dyn Error>> {
    mongo_db::update_game(id, command).await
}

pub async fn get_game_info(id: Uuid) -> Result<GameInfo, Box<dyn Error>> {
    mongo_db::get_game_info(&id).await
}

pub async fn create_room(game_id: &Uuid) -> Result<RoomInfo, Box<dyn Error>> {
    let game_info = mongo_db::get_game_info(game_id).await?;
    let room_id = create_room_id();

    let teams_info = [
        TeamInfo {
            name: "A".to_owned(),
            participants: vec![],
            score: 0,
        },
        TeamInfo {
            name: "B".to_owned(),
            participants: vec![],
            score: 0,
        },
    ]
    .to_vec();

    let room_info = RoomInfo {
        id: room_id,
        title: game_info.title,
        teams_info,
        questions: game_info.questions,
    };

    mongo_db::create_room(&room_info).await?;

    Ok(room_info)
}

fn create_room_id() -> String {
    create_id(6)
}

fn create_id(n: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .map(|c| c.to_ascii_lowercase())
        .collect()
}
