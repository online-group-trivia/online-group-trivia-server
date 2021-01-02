use uuid::Uuid;

pub fn create_room(request_body: &String) -> String {
    let my_uuid = Uuid::new_v4();
    format!("{{\"roomUrl\": \"{}\"}}", my_uuid)
}