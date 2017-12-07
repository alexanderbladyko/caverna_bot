use std::collections::HashMap;

use models::game::{Game, Player};

pub trait MoveAction {
    fn perform(self, game: &mut Game);
}

pub struct UpdateResources {
    pub player: String,
    pub update_hash: HashMap<String, u32>,
}

impl MoveAction for UpdateResources {
    fn perform(self, game: &mut Game) {
        game.players
            .iter_mut()
            .find(|p| p.name == self.player)
            .unwrap()
            .change_resources(self.update_hash);
    }
}

