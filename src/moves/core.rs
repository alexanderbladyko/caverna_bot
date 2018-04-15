use std::collections::{HashMap};

use clap::{SubCommand, Arg, App, ArgMatches};

use constants;
use models::game::{Game};
use actions::{MoveAction, Actions, UpdateResources, SetFirstPlayer};
use moves::config::{MovesConfig};
use moves::{constants as MovesConstants};

pub struct ActionsFromMove {
    pub move_name: String,
    pub actions: Actions,
}

pub fn collect_actions(game: &Game, moves_config: &MovesConfig, moves: Vec<&Move>) -> Vec<ActionsFromMove> {
    let actions = moves
        .iter()
        .flat_map(|&m| {
            m.get_all_actions(game.clone(), moves_config).into_iter().map(|a| {
                ActionsFromMove {
                    move_name: String::from(m.get_name()),
                    actions: a,
                }
            }).collect::<Vec<ActionsFromMove>>()
        })
        .collect();
    actions
}

pub trait Move {
    fn get_name(&self) -> &str;
    fn get_sub_command(&self) -> App<'static, 'static>;
    fn parse_args(&self, _args: &ArgMatches) -> HashMap<String, String> {
        HashMap::new()
    }
    fn get_all_actions(&self, game: Game, moves_config: &MovesConfig) -> Vec<Actions>;
    fn get_actions(&self, game: Game, moves_config: &MovesConfig, args: &HashMap<String, String>) -> Actions;
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
        MovesConstants::DRIFT_MINING
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name(MovesConstants::DRIFT_MINING)
            .about("Drift Mining")
            .arg(Arg::with_name("hall_slot"))
            .arg(Arg::with_name("room_slot"))
    }

    fn parse_args(&self, args: &ArgMatches) -> HashMap<String, String> {
        hash_map! {
            String::from("hall_slot") => String::from(args.value_of("hall_slot").unwrap()),
            String::from("room_slot") => String::from(args.value_of("room_slot").unwrap())
        }
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
            args: hash_map! {
                String::from("hall_slot") => String::new(),
                String::from("room_slot") => String::new()
            },
            actions,
        });
        result
    }

    fn get_actions(&self, game: Game, moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
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
            args: HashMap::new(),
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
        MovesConstants::LOGGING
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name(MovesConstants::LOGGING)
            .about("Logging")
            .arg(Arg::with_name("extraction"))
    }

    fn get_all_actions(&self, game: Game, _moves_config: &MovesConfig) -> Vec<Actions> {
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
            args: HashMap::new(),
            actions,
        });
        result
    }

    fn get_actions(&self, game: Game, _moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
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
            args: HashMap::new(),
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
        MovesConstants::WOOD_GATHERING
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name(MovesConstants::WOOD_GATHERING)
            .about("Wood Gathering")
    }

    fn get_all_actions(&self, game: Game, _moves_config: &MovesConfig) -> Vec<Actions> {
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
            args: HashMap::new(),
            actions,
        });
        result
    }

    fn get_actions(&self, game: Game, _moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
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
            args: HashMap::new(),
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
        MovesConstants::EXCAVATION
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name(MovesConstants::EXCAVATION)
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

    fn get_all_actions(&self, game: Game, _moves_config: &MovesConfig) -> Vec<Actions> {
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
            args: HashMap::new(),
            actions,
        });
        result
    }

    fn get_actions(&self, game: Game, _moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
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
            args: HashMap::new(),
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
        MovesConstants::SUPPLIES
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name(MovesConstants::SUPPLIES)
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
        result.push(Actions { args: HashMap::new(), actions });
        result
    }

    fn get_actions(&self, game: Game, moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
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
            args: HashMap::new(),
            actions,
        }
    }

    fn on_next_turn(&self, _game: &mut Game, _moves_config: &MovesConfig) {
    }
}

pub struct Clearing {}

impl Move for Clearing {
    fn get_name(&self) -> &str {
        MovesConstants::CLEARING
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name(MovesConstants::CLEARING)
            .about("Wood Gathering")
    }

    fn get_all_actions(&self, game: Game, _moves_config: &MovesConfig) -> Vec<Actions> {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(constants::ResourceType::Wood.str_key(), game.moves.clearing.wood);

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources { player: game.next, update_hash }));

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { args: HashMap::new(), actions });
        result
    }

    fn get_actions(&self, game: Game, _moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(constants::ResourceType::Wood.str_key(), game.moves.clearing.wood);

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources {
            player: game.next,
            update_hash,
        }));
        Actions {
            args: HashMap::new(),
            actions,
        }
    }

    fn on_next_turn(&self, _game: &mut Game, _moves_config: &MovesConfig) {
    }
}

pub struct StartingPlayer {}

impl Move for StartingPlayer {
    fn get_name(&self) -> &str {
        MovesConstants::STARTING_PLAYER
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name(MovesConstants::STARTING_PLAYER)
            .about("Starting player")
    }

    fn get_all_actions(&self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(constants::ResourceType::Gem.str_key(), moves_config.starting_player.gem);
        update_hash.insert(constants::ResourceType::Coal.str_key(), moves_config.starting_player.coal);
        update_hash.insert(constants::ResourceType::Food.str_key(), game.moves.starting_player.food);

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources { player: game.next.clone(), update_hash }));
        actions.push(Box::new(SetFirstPlayer { player: game.next.clone() }));

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { args: HashMap::new(), actions });
        result
    }

    fn get_actions(&self, game: Game, moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(constants::ResourceType::Gem.str_key(), moves_config.starting_player.gem);
        update_hash.insert(constants::ResourceType::Coal.str_key(), moves_config.starting_player.coal);
        update_hash.insert(constants::ResourceType::Food.str_key(), game.moves.starting_player.food);

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources {
            player: game.next.clone(),
            update_hash,
        }));
        actions.push(Box::new(SetFirstPlayer { player: game.next.clone() }));
        Actions {
            args: HashMap::new(),
            actions,
        }
    }

    fn on_next_turn(&self, game: &mut Game, moves_config: &MovesConfig) {
        game.moves.starting_player.food += moves_config.starting_player.food_incr;
    }
}