use std::collections::{HashMap};

use clap::{SubCommand, Arg, App};

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
    pub fn collect_actions(self, moves: Vec<Box<Move>>) -> Vec<Actions> {
        Vec::new()
    }
}

pub trait Move {
    fn get_actions(self, game: Game, moves_config: &MovesConfig) -> Vec<Actions>;
    fn on_next_turn(self, game: &mut Game, moves_config: &MovesConfig);
    fn get_sub_command(&self) -> App<'static, 'static>;
}

pub fn get_from_string(string: &str) -> &Move {
    match string {
        "drift_mining" => &DriftMining {},
        "logging" => &Logging {},
        "wood_gathering" => &WoodGathering {},
        "excavation" => &Excavation {},
        "supplies" => &Supplies {},
        "clearing" => &Clearing {},
        "starting_player" => &StartingPlayer {},
        &_ => panic!(format!("No move for {} found", string)),
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

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name("drift_mining")
            .about("Drift Mining")
            .arg(Arg::with_name("hall_slot"))
            .arg(Arg::with_name("room_slot"))
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

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name("logging")
            .about("Logging")
            .arg(Arg::with_name("extraction"))
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

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name("wood_gathering")
            .about("Wood Gathering")
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

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name("excavation")
            .about("Excavation")
            .arg(Arg::with_name("first_slot"))
            .arg(Arg::with_name("second_slot"))
            .arg(Arg::with_name("two_rooms"))
    }
}

pub struct Supplies {}

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

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name("supplies")
            .about("Supplies")
    }
}

pub struct Clearing {}

impl Move for Clearing {
    fn get_actions(self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(constants::ResourceType::Wood.str_key(), game.moves.clearing.wood);

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources { player: game.next, update_hash }));

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { weight: 0, actions });
        result
    }

    fn on_next_turn(self, game: &mut Game, moves_config: &MovesConfig) {
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name("wood_gathering")
            .about("Wood Gathering")
    }
}

pub struct StartingPlayer {}

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

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name("wood_gathering")
            .about("Wood Gathering")
    }
}