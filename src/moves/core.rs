use std::collections::{HashMap};

use constants;
use models::game::{Game};
use moves::actions::{MoveAction, Actions, UpdateResources};
use moves::config::{MovesConfig};

pub trait Move {
    fn get_actions(self, game: Game, moves_config: &MovesConfig) -> Vec<Actions>;
    fn on_next_turn(self, game: &mut Game, moves_config: &MovesConfig);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MovesData {
    pub drift_mining: DriftMining,
    pub logging: Logging,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriftMining {
    coal: u32,
}

impl Move for DriftMining {
    fn get_actions(self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(
            constants::ResourceType::Coal.str_key(), moves_config.drift_mining.coal_incr
        );

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources {
            player: game.next,
            update_hash,
        }));

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions {
            weight: 0,
            actions,
        });
        result
    }

    fn on_next_turn(self, game: &mut Game, moves_config: &MovesConfig) {
        game.moves.drift_mining.coal += moves_config.drift_mining.coal_incr;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logging {
    wood: u32,
}

impl Move for Logging {
    fn get_actions(self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(
            constants::ResourceType::Wood.str_key(), game.moves.logging.wood
        );

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources {
            player: game.next,
            update_hash,
        }));

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions {
            weight: 0,
            actions,
        });
        result
    }

    fn on_next_turn(self, game: &mut Game, moves_config: &MovesConfig) {
        game.moves.logging += match game.moves.logging.wood {
            0 => moves_config.logging.wood_incr,
            _ => moves_config.logging.secondary_wood_incr,
        }
    }
}

