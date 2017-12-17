use std::collections::HashMap;

use models::game::{Game, PlayerRoom, PlayerField};

pub trait MoveAction {
    fn perform(self, game: &mut Game);

    fn get_info(&self) -> String;
}

pub struct Actions {
    pub weight: i64,
    pub actions: Vec<Box<MoveAction>>,
}

impl Actions {
    pub fn get_info(&self) -> Vec<String> {
        self.actions
            .iter()
            .map(|a| a.get_info())
            .collect()
    }
}

pub struct UpdateResources {
    pub player: String,
    pub update_hash: HashMap<String, u32>,
}

impl MoveAction for UpdateResources {
    fn perform(self, game: &mut Game) {
        game.get_player_mut(&self.player).change_resources(self.update_hash);
    }

    fn get_info(&self) -> String {
        format!("Updating resources {:?}", self.update_hash)
    }
}

pub struct BuildRooms {
    pub player: String,
    pub rooms: Vec<PlayerRoom>,
}

impl MoveAction for BuildRooms {
    fn perform(self, game: &mut Game) {
        game.get_player_mut(&self.player).add_rooms(self.rooms);
    }

    fn get_info(&self) -> String {
        format!("Building rooms {:?}", self.rooms)
    }
}

pub struct BuildFields {
    pub player: String,
    pub fields: Vec<PlayerField>,
}

impl MoveAction for BuildFields {
    fn perform(self, game: &mut Game) {
        game.get_player_mut(&self.player).add_fields(self.fields);
    }

    fn get_info(&self) -> String {
        format!("Building fields {:?}", self.fields)
    }
}

pub struct SpawnGnome {
    pub player: String,
}

impl MoveAction for SpawnGnome {
    fn perform(self, game: &mut Game) {
        game.get_player_mut(&self.player).spawn_new_gnome();
    }

    fn get_info(&self) -> String {
        format!("Spawning gnome")
    }
}
