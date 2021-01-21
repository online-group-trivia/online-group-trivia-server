mod data_model;
mod database;
mod logic;
mod websocket;

use crate::data_model::GameInfo;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::{Json, Query};
use actix_web::{
    get, middleware, post, put, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use env_logger::Env;
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
        Ok(response_body) => HttpResponse::Ok().json(response_body),
        Err(_) => HttpResponse::InternalServerError().finish(),
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
        Ok(game_info) => HttpResponse::Ok().json(game_info),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[put("/save")]
async fn save(game_info: Json<GameInfo>) -> impl Responder {
    match logic::update_game(&game_info.0) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/start")]
async fn start(game_id: Json<Uuid>) -> impl Responder {
    match logic::create_room(&game_id.0) {
        Ok(room_info) => HttpResponse::Ok().json(room_info),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "0.0.0.0:9631";

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .wrap(middleware::DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
            .service(create)
            .service(manage)
            .service(save)
            .service(start)
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
    })
    .bind(address)?
    .run()
    .await
}
