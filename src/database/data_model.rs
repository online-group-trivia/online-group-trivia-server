use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RedisGameInfo {
    pub title: String,
    pub questions: Vec<String>,
}
