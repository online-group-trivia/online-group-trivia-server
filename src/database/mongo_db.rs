use crate::data_model::{GameInfo, RoomInfo};
use crate::database::data_model::{MyError, UpdateGameCommand};
use bson::document::Document;
use mongodb::{bson, bson::doc, bson::Bson, options::ClientOptions, Client, Collection, Database};
use serde::export::fmt::Debug;
use serde::Serialize;
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
    coll.insert_one(to_document(&game_info)?, None).await?;
    Ok(game_info)
}

pub fn to_document<T: ?Sized + Serialize + Debug>(value: &T) -> Result<Document, Box<dyn Error>> {
    let bson = bson::to_bson(value).unwrap();
    Ok(bson.as_document().unwrap().to_owned())
}

pub async fn get_game_info(game_id: &Uuid) -> Result<GameInfo, Box<dyn Error>> {
    let coll = get_collection_handler("games").await?;
    let game_info_doc = coll
        .find_one(doc! {"_id":game_id.to_string()}, None)
        .await?
        .expect("Document not found.");
    Ok(bson::from_bson(Bson::Document(game_info_doc))?)
}

pub async fn update_game(id: &Uuid, command: &UpdateGameCommand) -> Result<(), Box<dyn Error>> {
    let coll = get_collection_handler("games").await?;
    let mut result = 0;
    match command {
        UpdateGameCommand::AddQuestion { question } => {
            result = coll
                .update_one(
                    doc! {"_id":id.to_string()},
                    doc! {"$addToSet" : {"questions":question.to_owned()}},
                    None,
                )
                .await
                .unwrap()
                .matched_count;
        }
        UpdateGameCommand::RemoveQuestion { question } => {
            coll.update_one(
                doc! {"_id":id.to_string()},
                doc! {"$pull" : {"questions":question.to_owned()}},
                None,
            )
            .await
            .unwrap()
            .matched_count;
        }
        UpdateGameCommand::ChangeTitle { title } => {
            coll.update_one(
                doc! {"_id":id.to_string()},
                doc! {"$set" : {"title":title.to_owned()}},
                None,
            )
            .await
            .unwrap()
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
    coll.insert_one(to_document(&room_info)?, None).await?;
    Ok(())
}

async fn get_db_handler() -> Result<Database, Box<dyn Error>> {
    let client_options = ClientOptions::parse("mongodb://root:example@localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    Ok(client.database("app"))
}

async fn get_collection_handler(name: &str) -> Result<Collection, Box<dyn Error>> {
    let db = get_db_handler().await?;
    Ok(db.collection(name))
}
