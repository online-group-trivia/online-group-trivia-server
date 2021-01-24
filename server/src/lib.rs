use database::mongo_db;
use interfaces::{GameInfo, JoinRoomRequest, Participant, RoomInfo, TeamInfo, UpdateRoomCommand};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::error::Error;
use uuid::Uuid;

use interfaces::UpdateGameCommand;

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

pub async fn join_room(join_room_request: JoinRoomRequest) -> Result<RoomInfo, Box<dyn Error>> {
    let room_info = mongo_db::get_room_info(&join_room_request.id).await?;
    let smallest_team_idx = get_smallest_team_idx(&room_info);
    let join_room_cmd = UpdateRoomCommand::AddParticipant {
        participant: Participant {
            name: join_room_request.display_name,
        },
        team_index: smallest_team_idx,
    };

    Ok(mongo_db::update_room(&join_room_request.id, join_room_cmd).await?)
}

fn get_smallest_team_idx(room_info: &RoomInfo) -> usize {
    room_info
        .teams_info
        .iter()
        .enumerate()
        .min_by_key(|(_, team_info)| team_info.participants.len())
        .map(|(i, _)| i)
        .unwrap()
}
