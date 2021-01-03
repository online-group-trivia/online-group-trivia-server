mod logic;
mod database_logic;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Query;
use serde::{Deserialize, Serialize};

#[post("/create")]
async fn create() -> impl Responder {
    let response_body = logic::create_room();
    HttpResponse::Ok().body(response_body)
}

#[derive(Deserialize)]
struct RoomUuid {
    room_uuid: String,
}

#[get("/manage")]
async fn manage(room_uuid: Query<RoomUuid>) -> impl Responder {
    let room_info = database_logic::get_room_info(room_uuid.room_uuid.clone()).unwrap();
    HttpResponse::Ok().body(room_info)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "0.0.0.0:9631";
    let server = HttpServer::new(|| {
        App::new()
            .service(create)
            .service(manage)
    })
        .bind(address)?
        .run();
    println!("LLLLL Group trivia server has started and listening to {}", address);

    server.await
}
