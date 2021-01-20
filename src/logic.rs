use crate::data_model::{GameInfo, RoomInfo, TeamInfo, JoinRoomRequest, Participant};
use crate::database::database_logic;
use crate::{database, Uuid};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::error::Error;
use crate::database::data_model::RedisRoomInfo;

pub fn create_game(room_name: &String) -> Result<GameInfo, Box<dyn Error>> {
    database_logic::create_game(room_name)
}

pub fn update_game(game_info: &GameInfo) -> Result<(), Box<dyn Error>> {
    let db_game_info = database::data_model::RedisGameInfo {
        title: game_info.title.to_owned(),
        questions: game_info.questions.to_owned(),
    };
    database_logic::update_game(&game_info.id, &db_game_info)
}

pub fn get_game_info(id: Uuid) -> Result<GameInfo, Box<dyn Error>> {
    let redis_game_info = database_logic::get_game_info(&id)?;
    Ok(GameInfo {
        id,
        title: redis_game_info.title,
        questions: redis_game_info.questions,
    })
}

pub fn create_room(game_id: &Uuid) -> Result<RoomInfo, Box<dyn Error>> {
    let redis_game_info = database_logic::get_game_info(game_id)?;
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
        title: redis_game_info.title,
        teams_info,
        questions: redis_game_info.questions,
    };

    database_logic::create_room(&room_info)?;

    Ok(room_info)
}

pub fn join_room(join_room_request: &JoinRoomRequest) -> Result<RoomInfo, Box<dyn Error>> {
    let mut redis_room_info = database_logic::get_room_info(join_room_request.id.to_string())?;
    add_participant_to_room(&mut redis_room_info, &join_room_request);

    let room_info = RoomInfo::new(&join_room_request.id, redis_room_info);
    database_logic::create_room(&room_info)?;

    Ok(room_info)
}

fn add_participant_to_room(redis_room_info: &mut RedisRoomInfo, join_room_request: &JoinRoomRequest) {
    let team_to_add_to = redis_room_info.teams_info.iter_mut().min_by_key(|team_info| team_info.participants.len()).unwrap();
    println!("{:#?}", team_to_add_to);
    let participant = Participant {
        name: join_room_request.display_name.to_owned()
    };
    team_to_add_to.participants.push(participant);
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