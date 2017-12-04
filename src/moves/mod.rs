use std::collections::HashMap;

use models::game::{Game};

pub trait MoveAction {
    fn perform(self, game: &mut Game);
}

struct UpdateResources {
    player: String,
    update_hash: HashMap<String, i64>,
}

impl MoveAction for UpdateResources {
    fn perform(self, game: &mut Game) {
        game.turn += 1;

        let mut player_data = &game.players
            .into_iter()
            .find(|player| player.name == self.player)
            .unwrap();
        player_data.change_resources(self.update_hash);
    }
}