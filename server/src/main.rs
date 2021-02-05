mod lib;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::{Data, Json, Query};
use actix_web::{
    get, middleware, post, put, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use database;
use database::data_model;
use database::data_model::MongoDb;
use env_logger::Env;
use interfaces::{JoinRoomRequest, UpdateGameCommand};
use log::error;
use log::info;
use serde::Deserialize;
use std::sync::Mutex;
use uuid::Uuid;

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
async fn create(
    create_game_info: Json<CreateGameInfo>,
    app_state: Data<AppState>,
) -> impl Responder {
    match lib::create_game(&create_game_info.title, &app_state.db).await {
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
async fn manage(
    manage_game_query: Query<ManageGameQuery>,
    app_state: Data<AppState>,
) -> impl Responder {
    match lib::get_game_info(manage_game_query.game_id, &app_state.db).await {
        Ok(game_info) => HttpResponse::Ok().json(game_info),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[put("/save")]
async fn save(
    game_id: Query<ManageGameQuery>,
    update_game_command: Json<UpdateGameCommand>,
    app_state: Data<AppState>,
) -> impl Responder {
    match lib::update_game(&game_id.game_id, &update_game_command.0, &app_state.db).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/start")]
async fn start(game_id: Json<Uuid>, app_state: Data<AppState>) -> impl Responder {
    match lib::create_room(&game_id.0, &app_state.db).await {
        Ok(room_info) => HttpResponse::Ok().json(room_info),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/join")]
async fn join(
    join_room_request: Json<JoinRoomRequest>,
    app_state: Data<AppState>,
) -> impl Responder {
    match lib::join_room(join_room_request.0, &app_state.db).await {
        Ok(room_info) => HttpResponse::Ok().json(room_info),
        Err(err) => {
            error!("{}", err.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}

struct AppState {
    db: MongoDb,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "0.0.0.0:9631";

    let db = database::data_model::MongoDb::new().await;

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(move || {
        App::new()
            .data(AppState { db: db.clone() })
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
            .service(join)
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
    })
    .bind(address)?
    .run()
    .await
}
