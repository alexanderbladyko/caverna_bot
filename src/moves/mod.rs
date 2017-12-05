use std::collections::HashMap;

use models::game::{Game, Player};

pub trait MoveAction {
    fn perform(self, game: &mut Game, players: &mut Vec<Player>);
}

pub struct UpdateResources {
    pub player: String,
    pub update_hash: HashMap<String, i64>,
}

impl MoveAction for UpdateResources {
    fn perform(self, game: &mut Game, players: &mut Vec<Player>) {
        game.turn += 1;

        let mut player_data = players
            .into_iter()
            .find(|player| player.name == self.player)
            .unwrap();
        player_data.change_resources(self.update_hash);
    }
}