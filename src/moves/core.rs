    use std::collections::{HashMap};

use constants;
use models::game::{Game};
use moves::actions::{MoveAction, Actions, UpdateResources};
use moves::config::{MovesConfig};


pub struct Moves {
    pub actions: Vec<Actions>,
    pub game: Game,
    pub moves_config: MovesConfig,
}

impl Moves {
    pub fn collect_actions(self) -> Vec<Actions> {
        Vec::new()
    }
}

pub trait Move {
    fn get_actions(self, game: Game, moves_config: &MovesConfig) -> Vec<Actions>;
    fn on_next_turn(self, game: &mut Game, moves_config: &MovesConfig);
}

pub fn get_from_string(string: &str) -> Box<Move> {
    match string {
        "drift_mining" => Box::from(DriftMining {}),
        "logging" => Box::from(Logging {}),
        "wood_gathering" => Box::from(WoodGathering {}),
        "excavation" => Box::from(Excavation {}),
        &_ => panic!(format!("No struct for {}", string)),
    }
}

pub struct DriftMining {}

impl Move for DriftMining {
    fn get_actions(self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(
            constants::ResourceType::Stone.str_key(), moves_config.drift_mining.stone_incr
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
        game.moves.drift_mining.stone += moves_config.drift_mining.stone_incr;
    }
}

pub struct Logging {}

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
        game.moves.logging.wood += match game.moves.logging.wood {
            0 => moves_config.logging.wood_incr,
            _ => moves_config.logging.secondary_wood_incr,
        }
    }
}

pub struct WoodGathering {}

impl Move for WoodGathering {
    fn get_actions(self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(
            constants::ResourceType::Wood.str_key(), game.moves.wood_gathering.wood
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
        game.moves.wood_gathering.wood += moves_config.wood_gathering.wood_incr;
    }
}

pub struct Excavation {}

impl Move for Excavation {
    fn get_actions(self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(
            constants::ResourceType::Stone.str_key(), game.moves.excavation.stone
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
        game.moves.excavation.stone += moves_config.excavation.stone_incr;
    }
}

