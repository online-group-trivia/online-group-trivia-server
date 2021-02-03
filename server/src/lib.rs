use database::data_model;
use interfaces::{GameInfo, JoinRoomRequest, Participant, RoomInfo, TeamInfo, UpdateRoomCommand};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::error::Error;
use uuid::Uuid;

use interfaces::UpdateGameCommand;

pub async fn create_game(
    title: &String,
    db: &Box<dyn data_model::GroupTriviaDatabase + Send + 'static>,
) -> Result<GameInfo, Box<dyn Error>> {
    (*db).create_game(title).await
}

pub async fn update_game(
    id: &Uuid,
    command: &UpdateGameCommand,
    db: &Box<dyn data_model::GroupTriviaDatabase + Send + 'static>,
) -> Result<(), Box<dyn Error>> {
    (*db).update_game(id, command).await
}

pub async fn get_game_info(
    id: Uuid,
    db: &Box<dyn data_model::GroupTriviaDatabase + Send + 'static>,
) -> Result<GameInfo, Box<dyn Error>> {
    (*db).get_game_info(&id).await
}

pub async fn create_room(
    game_id: &Uuid,
    db: &Box<dyn data_model::GroupTriviaDatabase + Send + 'static>,
) -> Result<RoomInfo, Box<dyn Error>> {
    let game_info = (*db).get_game_info(game_id).await?;
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

    (*db).create_room(&room_info).await?;

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

pub async fn join_room(
    join_room_request: JoinRoomRequest,
    db: &Box<dyn data_model::GroupTriviaDatabase + Send + 'static>,
) -> Result<RoomInfo, Box<dyn Error>> {
    let room_info = (*db).get_room_info(&join_room_request.id).await?;
    let smallest_team_idx = get_smallest_team_idx(&room_info);
    let join_room_cmd = UpdateRoomCommand::AddParticipant {
        participant: Participant {
            name: join_room_request.display_name,
        },
        team_index: smallest_team_idx,
    };

    Ok((*db)
        .update_room(&join_room_request.id, join_room_cmd)
        .await?)
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
