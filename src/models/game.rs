use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub resources: HashMap<String, i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub turn: i64,
    pub players: Vec<Player>,
}