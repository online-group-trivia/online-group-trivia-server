use crate::data_model::MyError;
use interfaces::{GameInfo, RoomInfo, UpdateGameCommand, UpdateRoomCommand};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use mongodb::{bson, bson::doc, bson::Bson, options::ClientOptions, Client, Collection, Database};
use std::error::Error;
use uuid::Uuid;

pub async fn create_game(title: &String) -> Result<GameInfo, Box<dyn Error>> {
    let id = Uuid::new_v4();
    let coll = get_collection_handler("games").await?;
    let game_info = GameInfo {
        id,
        title: title.to_owned(),
        questions: vec![],
    };
    coll.insert_one(bson::to_document(&game_info)?, None)
        .await?;
    Ok(game_info)
}

pub async fn get_game_info(game_id: &Uuid) -> Result<GameInfo, Box<dyn Error>> {
    let coll = get_collection_handler("games").await?;
    let game_info_doc = coll
        .find_one(doc! {"_id":game_id.to_string()}, None)
        .await?
        .ok_or("Game not found.")?;
    Ok(bson::from_bson(Bson::Document(game_info_doc))?)
}

pub async fn update_game(id: &Uuid, command: &UpdateGameCommand) -> Result<(), Box<dyn Error>> {
    let coll = get_collection_handler("games").await?;
    let result;
    match command {
        UpdateGameCommand::AddQuestion { question } => {
            result = coll
                .update_one(
                    doc! {"_id":id.to_string()},
                    doc! {"$addToSet" : {"questions":question.to_owned()}},
                    None,
                )
                .await?
                .matched_count;
        }
        UpdateGameCommand::RemoveQuestion { question } => {
            result = coll
                .update_one(
                    doc! {"_id":id.to_string()},
                    doc! {"$pull" : {"questions":question.to_owned()}},
                    None,
                )
                .await?
                .matched_count;
        }
        UpdateGameCommand::ChangeTitle { title } => {
            result = coll
                .update_one(
                    doc! {"_id":id.to_string()},
                    doc! {"$set" : {"title":title.to_owned()}},
                    None,
                )
                .await?
                .matched_count;
        }
    }

    if result == 0 {
        return Err(Box::new(MyError("Oops".into())));
    }

    Ok(())
}

pub async fn create_room(room_info: &RoomInfo) -> Result<(), Box<dyn Error>> {
    let coll = get_collection_handler("rooms").await?;
    coll.insert_one(bson::to_document(&room_info)?, None)
        .await?;
    Ok(())
}

pub async fn get_room_info(id: &String) -> Result<RoomInfo, Box<dyn Error>> {
    let rooms = get_collection_handler("rooms").await?;
    let room_info_doc = rooms
        .find_one(doc! {"_id":id}, None)
        .await?
        .ok_or("Room not found.")?;

    Ok(bson::from_bson(Bson::Document(room_info_doc))?)
}

async fn get_db_handler() -> Result<Database, Box<dyn Error>> {
    let client_options = ClientOptions::parse("mongodb://root:example@mongo:27017").await?;
    let client = Client::with_options(client_options)?;
    Ok(client.database("app"))
}

async fn get_collection_handler(name: &str) -> Result<Collection, Box<dyn Error>> {
    let db = get_db_handler().await?;
    Ok(db.collection(name))
}

pub async fn update_room(
    id: &String,
    command: UpdateRoomCommand,
) -> Result<RoomInfo, Box<dyn Error>> {
    let coll = get_collection_handler("rooms").await?;
    let result;
    match command {
        UpdateRoomCommand::AddParticipant {
            participant,
            team_index,
        } => {
            let s = format!("teamsInfo.{}.participants", team_index);
            result = coll
                .find_one_and_update(
                    doc! {"_id":id.to_string()},
                    doc! {"$addToSet" : {s: bson::to_bson(&participant)?}},
                    Some(
                        FindOneAndUpdateOptions::builder()
                            .return_document(ReturnDocument::After)
                            .build(),
                    ),
                )
                .await?
                .ok_or("Failed to retrieve document.")?;
        }
    }
    Ok(bson::from_bson(bson::Bson::Document(result))?)
}
