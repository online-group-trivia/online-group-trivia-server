mod logic;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[post("/create")]
async fn create(req_body: String) -> impl Responder {
    let response_body = logic::create_room(&req_body);
    HttpResponse::Ok().body(response_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "0.0.0.0:9631";
    let server = HttpServer::new(|| {
        App::new()
            .service(create)
    })
        .bind(address)?
        .run();
    println!("Group trivia server has started and listening to {}", address);

    server.await
}
