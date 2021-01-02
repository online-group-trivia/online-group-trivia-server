use uuid::Uuid;

extern crate redis;

use redis::Commands;

pub fn create_room(request_body: &String) -> String {

    let my_uuid = Uuid::new_v4();
    let client = redis::Client::open("redis://redis:6379").unwrap();
    let mut con = client.get_connection().unwrap();
    let _: () = con.incr("api_count_create", 1).unwrap();
    format!("{{\"roomUrl\": \"{}\"}}", my_uuid)
}