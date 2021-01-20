mod data_model;
mod database;
mod logic;
mod websocket;

use std::error;
use crate::data_model::{GameInfo, RoomInfo, JoinRoomRequest};
use actix_cors::Cors;
use actix_web::web::{Json, Query};
use actix_web::{
    get, post, put, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use serde::Deserialize;
use uuid::Uuid;

extern crate simple_error;

async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", r);
    let res = ws::start(websocket::websocket_handler::MyWebSocket::new(), &r, stream);
    println!("{:?}", res);
    res
}

#[derive(Deserialize)]
struct CreateGameInfo {
    title: String,
}

#[post("/create")]
async fn create(create_game_info: Json<CreateGameInfo>) -> impl Responder {
    match logic::create_game(&create_game_info.title) {
        Ok(response_body) => HttpResponse::Ok()
            .header("Access-Control-Allow-Origin", "*")
            .json(response_body),
        Err(_) => HttpResponse::InternalServerError()
            .header("Access-Control-Allow-Origin", "*")
            .finish(),
    }
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize)]
struct ManageGameQuery {
    game_id: Uuid,
}

#[get("/manage")]
async fn manage(manage_game_query: Query<ManageGameQuery>) -> impl Responder {
    match logic::get_game_info(manage_game_query.game_id) {
        Ok(game_info) => HttpResponse::Ok()
            .header("Access-Control-Allow-Origin", "*")
            .json(game_info),
        Err(_) => HttpResponse::NotFound()
            .header("Access-Control-Allow-Origin", "*")
            .finish(),
    }
}

#[put("/save")]
async fn save(game_info: Json<GameInfo>) -> impl Responder {
    match logic::update_game(&game_info.0) {
        Ok(_) => HttpResponse::Ok()
            .header("Access-Control-Allow-Origin", "*")
            .finish(),
        Err(_) => HttpResponse::InternalServerError()
            .header("Access-Control-Allow-Origin", "*")
            .finish(),
    }
}

#[post("/start")]
async fn start(game_id: Json<Uuid>) -> impl Responder {
    room_response(logic::create_room(&game_id.0))
}

#[post("/join")]
async fn join(join_room_request: Json<JoinRoomRequest>) -> impl Responder {
    room_response(logic::join_room(&join_room_request.0))
}

fn room_response(room_info_result: Result<RoomInfo, Box<dyn error::Error>>) -> impl Responder {
    match room_info_result {
        Ok(room_info) => HttpResponse::Ok()
            .header("Access-Control-Allow-Origin", "*")
            .json(room_info),
        Err(_) => HttpResponse::InternalServerError()
            .header("Access-Control-Allow-Origin", "*")
            .finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "0.0.0.0:9631";
    let server = HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(create)
            .service(manage)
            .service(save)
            .service(start)
            .service(join)
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
    })
        .bind(address)?
        .run();
    println!(
        "Group trivia server has started and listening to {}",
        address
    );

    server.await
}
