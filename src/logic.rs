extern crate redis;
use uuid::Uuid;
use crate::database_logic;

pub fn create_room() -> String {
    let my_uuid = Uuid::new_v4();
    database_logic::create_room(my_uuid.to_string()).expect("abxc");
    format!("{{\"room_uuid\": \"{}\"}}", my_uuid)
}