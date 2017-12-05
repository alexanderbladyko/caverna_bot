use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub resources: HashMap<String, i64>,
}

impl Player {
    pub fn change_resources(&mut self, delta: HashMap<String, i64>) {
        for key in delta.keys().into_iter() {
            *self.resources.get_mut(key).unwrap() += *delta.get(key).unwrap();
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub turn: i64,
    pub next: String,
    pub first_move: String,
    pub order: Vec<String>,
    pub players: Vec<Player>,
}

