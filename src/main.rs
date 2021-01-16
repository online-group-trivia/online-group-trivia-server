mod database_logic;
mod logic;
mod websocket;

use actix_cors::Cors;
use actix_web::web::{Json, Query};
use actix_web::{
    get, post, put, web, web::Bytes, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use serde::Deserialize;

extern crate simple_error;

async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", r);
    let res = ws::start(websocket::websocket_handler::MyWebSocket::new(), &r, stream);
    println!("{:?}", res);
    res
}

#[derive(Deserialize)]
struct CreateRoomInfo {
    title: String,
}

#[post("/create")]
async fn create(create_room_info: Json<CreateRoomInfo>) -> impl Responder {
    match logic::create_room(&create_room_info.title) {
        Ok(response_body) => HttpResponse::Ok()
            .header("Access-Control-Allow-Origin", "*")
            .json(response_body),
        Err(_) => HttpResponse::InternalServerError()
            .header("Access-Control-Allow-Origin", "*")
            .finish(),
    }
}

#[derive(Deserialize)]
struct ManageRoomQuery {
    room_uuid: String,
}

#[get("/manage")]
async fn manage(manage_room_query: Query<ManageRoomQuery>) -> impl Responder {
    match database_logic::get_game_info(manage_room_query.room_uuid.clone()) {
        Ok(room_info) => HttpResponse::Ok()
            .header("Access-Control-Allow-Origin", "*")
            .body(room_info),
        Err(_) => HttpResponse::NotFound()
            .header("Access-Control-Allow-Origin", "*")
            .finish(),
    }
}

#[put("/save")]
async fn save(bytes: Bytes, room_uuid: Query<ManageRoomQuery>) -> impl Responder {
    match database_logic::update_game(
        room_uuid.room_uuid.clone(),
        String::from_utf8(bytes.to_vec()).unwrap(),
    ) {
        Ok(_) => HttpResponse::Ok()
            .header("Access-Control-Allow-Origin", "*")
            .finish(),
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
