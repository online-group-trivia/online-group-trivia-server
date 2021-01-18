use crate::data_model::{GameInfo, RoomInfo, TeamInfo};
use crate::database_logic;
use crate::Uuid;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::error::Error;

pub fn create_game(room_name: &String) -> Result<GameInfo, Box<dyn Error>> {
    database_logic::create_game(room_name)
}

pub fn create_room(game_id: &Uuid) -> Result<RoomInfo, Box<dyn Error>> {
    let game_info = database_logic::get_game_info(game_id)?;

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
        title: "".to_owned(),
        teams_info,
        questions: game_info.questions,
    };

    database_logic::create_room(&room_info);

    Ok(room_info)
}

fn create_room_id() -> String {
    create_id(4)
}

fn create_id(n: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}
