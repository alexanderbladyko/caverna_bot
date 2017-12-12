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
    pub wood_gathering: WoodGathering,
    pub excavation: Excavation,
    pub starting_player: StartingPlayer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriftMining {
    stone: u32,
}

impl Move for DriftMining {
    fn get_actions(self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(
            constants::ResourceType::Coal.str_key(), moves_config.drift_mining.stone_incr
        );

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources { player: game.next, update_hash }));

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { weight: 0, actions });
        result
    }

    fn on_next_turn(self, game: &mut Game, moves_config: &MovesConfig) {
        game.moves.drift_mining.stone += moves_config.drift_mining.stone_incr;
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
        actions.push(Box::new(UpdateResources { player: game.next, update_hash }));

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { weight: 0, actions });
        result
    }

    fn on_next_turn(self, game: &mut Game, moves_config: &MovesConfig) {
        game.moves.logging.wood += match game.moves.logging.wood {
            0 => moves_config.logging.wood_incr,
            _ => moves_config.logging.secondary_wood_incr,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WoodGathering {
    wood: u32,
}

impl Move for WoodGathering {
    fn get_actions(self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(
            constants::ResourceType::Wood.str_key(), game.moves.wood_gathering.wood
        );

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources { player: game.next, update_hash }));

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { weight: 0, actions });
        result
    }

    fn on_next_turn(self, game: &mut Game, moves_config: &MovesConfig) {
        game.moves.wood_gathering.wood += moves_config.wood_gathering.wood_incr;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Excavation {
    stone: u32,
}

impl Move for Excavation {
    fn get_actions(self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(
            constants::ResourceType::Stone.str_key(), game.moves.excavation.stone
        );

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources { player: game.next, update_hash }));

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { weight: 0, actions });
        result
    }

    fn on_next_turn(self, game: &mut Game, moves_config: &MovesConfig) {
        game.moves.excavation.stone += moves_config.excavation.stone_incr;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Supplies {
}

impl Move for Supplies {
    fn get_actions(self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(constants::ResourceType::Stone.str_key(), moves_config.supplies.stone);
        update_hash.insert(constants::ResourceType::Wood.str_key(), moves_config.supplies.wood);
        update_hash.insert(constants::ResourceType::Coal.str_key(), moves_config.supplies.coal);
        update_hash.insert(constants::ResourceType::Food.str_key(), moves_config.supplies.food);
        update_hash.insert(constants::ResourceType::Gold.str_key(), moves_config.supplies.gold);

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources { player: game.next, update_hash }));

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { weight: 0, actions });
        result
    }

    fn on_next_turn(self, game: &mut Game, moves_config: &MovesConfig) {
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartingPlayer {
    pub food: u32,
}

impl Move for StartingPlayer {
    fn get_actions(self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(constants::ResourceType::Gem.str_key(), moves_config.starting_player.gem);
        update_hash.insert(constants::ResourceType::Coal.str_key(), moves_config.starting_player.coal);
        update_hash.insert(constants::ResourceType::Food.str_key(), game.moves.starting_player.food);

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources { player: game.next, update_hash }));

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { weight: 0, actions });
        result
    }

    fn on_next_turn(self, game: &mut Game, moves_config: &MovesConfig) {
        game.moves.starting_player.food += moves_config.starting_player.food_incr;
    }
}