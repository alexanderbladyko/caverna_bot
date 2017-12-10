use std::collections::HashMap;

use models::game::{Game, PlayerRoom, PlayerField};

pub trait MoveAction {
    fn perform(self, game: &mut Game);
}

pub struct Actions {
    pub weight: i64,
    pub actions: Vec<Box<MoveAction>>,
}

pub struct UpdateResources {
    pub player: String,
    pub update_hash: HashMap<String, u32>,
}

impl MoveAction for UpdateResources {
    fn perform(self, game: &mut Game) {
        game.get_player_mut(&self.player).change_resources(self.update_hash);
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
}

pub struct BuildFields {
    pub player: String,
    pub fields: Vec<PlayerField>,
}

impl MoveAction for BuildFields {
    fn perform(self, game: &mut Game) {
        game.get_player_mut(&self.player).add_fields(self.fields);
    }
}

pub struct SpawnGnome {
    pub player: String,
}

impl MoveAction for SpawnGnome {
    fn perform(self, game: &mut Game) {
        game.get_player_mut(&self.player).spawn_new_gnome();
    }
}