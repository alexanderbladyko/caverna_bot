use std::collections::{HashMap};

use clap::{SubCommand, Arg, App, ArgMatches};

use constants;
use models::game::{Game};
use moves::actions::{MoveAction, Actions, UpdateResources};
use moves::config::{MovesConfig};


pub fn collect_actions(game: &Game, moves_config: &MovesConfig, moves: Vec<&Move>) -> Vec<Actions> {
    let mut actions = Vec::new();
    moves
        .iter()
        .for_each(|m| actions.extend(m.get_all_actions(game.clone(), moves_config)));
    actions
}

pub trait Move {
    fn get_name(&self) -> &str;
    fn get_sub_command(&self) -> App<'static, 'static>;
    fn get_all_actions(&self, game: Game, moves_config: &MovesConfig) -> Vec<Actions>;
    fn get_actions(&self, game: Game, moves_config: &MovesConfig, args: &ArgMatches) -> Actions;
    fn on_next_turn(&self, game: &mut Game, moves_config: &MovesConfig);
}

pub fn get_from_string(string: &str) -> Result<&Move, String> {
    let mut moves: HashMap<&str, &Move> = HashMap::new();

    moves.insert(DriftMining {}.get_name(), &DriftMining {});
    moves.insert(Logging {}.get_name(), &Logging {});
    moves.insert(WoodGathering {}.get_name(), &WoodGathering {});
    moves.insert(Excavation {}.get_name(), &Excavation {});
    moves.insert(Supplies {}.get_name(), &Supplies {});
    moves.insert(Clearing {}.get_name(), &Clearing {});
    moves.insert(StartingPlayer {}.get_name(), &StartingPlayer {});

    match moves.get(string) {
        Some(x) => Ok(*x),
        None => Err(format!("No move for {} found", string)),
    }
}

pub struct DriftMining {}

impl Move for DriftMining {
    fn get_name(&self) -> &str {
        "drift_mining"
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name("drift_mining")
            .about("Drift Mining")
            .arg(Arg::with_name("hall_slot"))
            .arg(Arg::with_name("room_slot"))
    }

    fn get_all_actions(&self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
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

    fn get_actions(&self, game: Game, moves_config: &MovesConfig, args: &ArgMatches) -> Actions {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(
            constants::ResourceType::Stone.str_key(), moves_config.drift_mining.stone_incr
        );

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources {
            player: game.next,
            update_hash,
        }));
        Actions {
            weight: 0,
            actions,
        }
    }

    fn on_next_turn(&self, game: &mut Game, moves_config: &MovesConfig) {
        game.moves.drift_mining.stone += moves_config.drift_mining.stone_incr;
    }
}

pub struct Logging {}

impl Move for Logging {
    fn get_name(&self) -> &str {
        "logging"
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name("logging")
            .about("Logging")
            .arg(Arg::with_name("extraction"))
    }

    fn get_all_actions(&self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
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

    fn get_actions(&self, game: Game, moves_config: &MovesConfig, args: &ArgMatches) -> Actions {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(
            constants::ResourceType::Wood.str_key(), game.moves.logging.wood
        );

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources {
            player: game.next,
            update_hash,
        }));
        Actions {
            weight: 0,
            actions,
        }
    }

    fn on_next_turn(&self, game: &mut Game, moves_config: &MovesConfig) {
        game.moves.logging.wood += match game.moves.logging.wood {
            0 => moves_config.logging.wood_incr,
            _ => moves_config.logging.secondary_wood_incr,
        }
    }
}

pub struct WoodGathering {}

impl Move for WoodGathering {
    fn get_name(&self) -> &str {
        "wood_gathering"
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name("wood_gathering")
            .about("Wood Gathering")
    }

    fn get_all_actions(&self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
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

    fn get_actions(&self, game: Game, moves_config: &MovesConfig, args: &ArgMatches) -> Actions {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(
            constants::ResourceType::Wood.str_key(), game.moves.wood_gathering.wood
        );

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources {
            player: game.next,
            update_hash,
        }));
        Actions {
            weight: 0,
            actions,
        }
    }

    fn on_next_turn(&self, game: &mut Game, moves_config: &MovesConfig) {
        game.moves.wood_gathering.wood += moves_config.wood_gathering.wood_incr;
    }
}

pub struct Excavation {}

impl Excavation {
    fn _is_valid_slot(s: String) -> Result<(), String> {
        if let Err(..) = s.parse::<u32>() {
            return Err(String::from("Not a valid number!"));
        }
        Ok(())
    }
}

impl Move for Excavation {
    fn get_name(&self) -> &str {
        "excavation"
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name("excavation")
            .about("Excavation")
            .arg(Arg::with_name("room_slot")
                .required(true)
                .help("Index of room slot")
                .short("r")
                .takes_value(true)
                .validator(Excavation::_is_valid_slot))
            .arg(Arg::with_name("second_slot")
                .required(true)
                .help("Index of hall/room slot. Hall by default")
                .short("h")
                .takes_value(true)
                .validator(Excavation::_is_valid_slot))
            .arg(Arg::with_name("two_rooms")
                .short("t")
                .help("Second slot will be room"))
    }

    fn get_all_actions(&self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
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

    fn get_actions(&self, game: Game, moves_config: &MovesConfig, args: &ArgMatches) -> Actions {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(
            constants::ResourceType::Stone.str_key(), game.moves.excavation.stone
        );

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources {
            player: game.next,
            update_hash,
        }));
        Actions {
            weight: 0,
            actions,
        }
    }

    fn on_next_turn(&self, game: &mut Game, moves_config: &MovesConfig) {
        game.moves.excavation.stone += moves_config.excavation.stone_incr;
    }
}

pub struct Supplies {}

impl Move for Supplies {
    fn get_name(&self) -> &str {
        "supplies"
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name("supplies")
            .about("Supplies")
    }

    fn get_all_actions(&self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
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

    fn get_actions(&self, game: Game, moves_config: &MovesConfig, args: &ArgMatches) -> Actions {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(constants::ResourceType::Stone.str_key(), moves_config.supplies.stone);
        update_hash.insert(constants::ResourceType::Wood.str_key(), moves_config.supplies.wood);
        update_hash.insert(constants::ResourceType::Coal.str_key(), moves_config.supplies.coal);
        update_hash.insert(constants::ResourceType::Food.str_key(), moves_config.supplies.food);
        update_hash.insert(constants::ResourceType::Gold.str_key(), moves_config.supplies.gold);

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources {
            player: game.next,
            update_hash,
        }));
        Actions {
            weight: 0,
            actions,
        }
    }

    fn on_next_turn(&self, game: &mut Game, moves_config: &MovesConfig) {
    }
}

pub struct Clearing {}

impl Move for Clearing {
    fn get_name(&self) -> &str {
        "clearing"
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name("wood_gathering")
            .about("Wood Gathering")
    }

    fn get_all_actions(&self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(constants::ResourceType::Wood.str_key(), game.moves.clearing.wood);

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources { player: game.next, update_hash }));

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { weight: 0, actions });
        result
    }

    fn get_actions(&self, game: Game, moves_config: &MovesConfig, args: &ArgMatches) -> Actions {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(constants::ResourceType::Wood.str_key(), game.moves.clearing.wood);

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources {
            player: game.next,
            update_hash,
        }));
        Actions {
            weight: 0,
            actions,
        }
    }

    fn on_next_turn(&self, game: &mut Game, moves_config: &MovesConfig) {
    }
}

pub struct StartingPlayer {}

impl Move for StartingPlayer {
    fn get_name(&self) -> &str {
        "starting_player"
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name("starting_player")
            .about("Starting player")
    }

    fn get_all_actions(&self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
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

    fn get_actions(&self, game: Game, moves_config: &MovesConfig, args: &ArgMatches) -> Actions {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(constants::ResourceType::Gem.str_key(), moves_config.starting_player.gem);
        update_hash.insert(constants::ResourceType::Coal.str_key(), moves_config.starting_player.coal);
        update_hash.insert(constants::ResourceType::Food.str_key(), game.moves.starting_player.food);

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources {
            player: game.next,
            update_hash,
        }));
        Actions {
            weight: 0,
            actions,
        }
    }

    fn on_next_turn(&self, game: &mut Game, moves_config: &MovesConfig) {
        game.moves.starting_player.food += moves_config.starting_player.food_incr;
    }
}