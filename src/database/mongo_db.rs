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
        UpdateGameCommand::AddQuestion(q) => {
            result = coll
                .update_one(
                    doc! {"_id":id.to_string()},
                    doc! {"$addToSet" : {"questions":q.to_owned()}},
                    None,
                )
                .await
                .unwrap()
                .matched_count;
        }
        UpdateGameCommand::RemoveQuestion(q) => {
            coll.update_one(
                doc! {"_id":id.to_string()},
                doc! {"$pull" : {"questions":q.to_owned()}},
                None,
            )
            .await
            .unwrap()
            .matched_count;
        }
        UpdateGameCommand::ChangeTitle(t) => {
            coll.update_one(
                doc! {"_id":id.to_string()},
                doc! {"$set" : {"title":t.to_owned()}},
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create;
    use crate::data_model::{Participant, TeamInfo};
    use crate::database::data_model::UpdateGameCommand::{
        AddQuestion, ChangeTitle, RemoveQuestion,
    };
    use std::str::FromStr;

    #[actix_rt::test]
    async fn connection_opened_successfully() {
        get_collection_handler("games").await.unwrap();
    }

    #[actix_rt::test]
    async fn create_game_test() {
        create_game(&"My cool game".to_string()).await.unwrap();
    }

    #[actix_rt::test]
    async fn create_room_test() {
        create_room(&RoomInfo {
            id: "12345".to_owned(),
            title: "my-title".to_string(),
            teams_info: vec![
                TeamInfo {
                    name: "A".to_string(),
                    participants: vec![Participant {
                        name: "Avi".to_owned(),
                    }],
                    score: 50,
                },
                TeamInfo {
                    name: "B".to_string(),
                    participants: vec![Participant {
                        name: "John".to_owned(),
                    }],
                    score: 50,
                },
            ],
            questions: vec!["Question 1?".to_owned(), "Question 2?".to_owned()],
        })
        .await
        .unwrap();
    }

    #[actix_rt::test]
    async fn add_question_test() {
        update_game(
            &Uuid::from_str("a13c9ef9-d945-4172-b531-5a378bc7ae3e").unwrap(),
            &AddQuestion(String::from("Question 4?")),
        )
        .await
        .unwrap();
    }

    #[actix_rt::test]
    async fn remove_question_test() {
        update_game(
            &Uuid::from_str("a13c9ef9-d945-4172-b531-5a378bc7ae3e").unwrap(),
            &RemoveQuestion(String::from("why?")),
        )
        .await
        .unwrap();
    }

    #[actix_rt::test]
    async fn change_title_test() {
        update_game(
            &Uuid::from_str("a13c9ef9-d945-4172-b531-5a378bc7af3e").unwrap(),
            &ChangeTitle(String::from("my-new-title")),
        )
        .await
        .unwrap();
    }
}
