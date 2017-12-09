use models::game::{Game};

use moves::actions::{Actions};
use moves::config::{MovesConfig};

pub trait Move {
    fn get_actions(self, game: Game) -> Vec<Actions>;
    fn on_next_turn(self, game: &mut Game, moves_config: &MovesConfig);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriftMining {
    coal: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MovesData {
    pub drift_mining: DriftMining
}

impl Move for DriftMining {
    fn get_actions(self, game: Game) -> Vec<Actions> {
        let result = Vec::new();

        result
    }

    fn on_next_turn(self, game: &mut Game, moves_config: &MovesConfig) {
        game.moves.drift_mining.coal += moves_config.drift_mining.coal_incr;
    }
}