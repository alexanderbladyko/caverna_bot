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

pub fn get_from_string(string: &str) -> Result<&Move, String> {
    let moves_list: Vec<&Move> = vec![
        &DriftMining {},
        &Logging {},
        &WoodGathering {},
        &Excavation {},
        &Supplies {},
        &Clearing {},
        &StartingPlayer {},
        &RubyMining {},
        &Housework {},
        &SlashAndBurn {},
        &Blacksmithing {},
        &SheepFarming {},
        &OreMineConstruction {},
        &WishForChildren {},
        &DonkeyFarming {},
        &RubyMineConstruction {},
        &FamilyLife {},
        &OreDelivery {},
        &Adventure {},
        &OreTrading {},
        &RubyDelivery {},
    ];

    match moves_list.into_iter().find(|m| m.get_name() == string) {
        Some(x) => Ok(x),
        None => Err(format!("No move for {} found", string)),
    }
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
    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name(self.get_name())
    }
    fn parse_args(&self, _args: &ArgMatches) -> HashMap<String, String> {
        HashMap::new()
    }
    fn get_all_actions(&self, game: Game, moves_config: &MovesConfig) -> Vec<Actions>;
    fn get_actions(&self, game: Game, moves_config: &MovesConfig, args: &HashMap<String, String>) -> Actions;
    fn on_next_turn(&self, _game: &mut Game, _moves_config: &MovesConfig) {}
}

pub struct DriftMining {}

impl Move for DriftMining {
    fn get_name(&self) -> &str {
        MovesConstants::DRIFT_MINING
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name(self.get_name())
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
        SubCommand::with_name(self.get_name())
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
        SubCommand::with_name(self.get_name())
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
        SubCommand::with_name(self.get_name())
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
        SubCommand::with_name(self.get_name())
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
}

pub struct Clearing {}

impl Move for Clearing {
    fn get_name(&self) -> &str {
        MovesConstants::CLEARING
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name(self.get_name())
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
}

pub struct StartingPlayer {}

impl Move for StartingPlayer {
    fn get_name(&self) -> &str {
        MovesConstants::STARTING_PLAYER
    }

    fn get_sub_command(&self) -> App<'static, 'static> {
        SubCommand::with_name(self.get_name())
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

pub struct RubyMining {}

impl Move for RubyMining {
    fn get_name(&self) -> &str {
        MovesConstants::RUBY_MINING
    }

    fn get_all_actions(&self, game: Game, moves_config: &MovesConfig) -> Vec<Actions> {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        // TODO: add condition if player has gem mines
        let mut gems = moves_config.ruby_mining.gems;
        gems += game.moves.ruby_mining.gems;
        update_hash.insert(constants::ResourceType::Gem.str_key(), gems);

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources { player: game.next.clone(), update_hash }));

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { args: HashMap::new(), actions });
        result
    }

    fn get_actions(&self, game: Game, moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        // TODO: add condition if player has gem mines
        let mut gems = moves_config.ruby_mining.gems;
        gems += game.moves.ruby_mining.gems;
        update_hash.insert(constants::ResourceType::Gem.str_key(), gems);

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources {
            player: game.next.clone(),
            update_hash,
        }));
        Actions {
            args: HashMap::new(),
            actions,
        }
    }

    fn on_next_turn(&self, game: &mut Game, moves_config: &MovesConfig) {
        if game.turn > moves_config.ruby_mining.from_turn {
            game.moves.ruby_mining.gems += moves_config.ruby_mining.gem_incr;
        }

    }
}

pub struct Housework {}

impl Move for Housework {
    fn get_name(&self) -> &str {
        MovesConstants::HOUSEWORK
    }

    fn get_all_actions(&self, game: Game, _moves_config: &MovesConfig) -> Vec<Actions> {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(constants::ResourceType::Dog.str_key(), 1);

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources { player: game.next.clone(), update_hash }));

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { args: HashMap::new(), actions });
        result
    }

    fn get_actions(&self, game: Game, _moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
        let mut update_hash: HashMap<String, u32> = HashMap::new();
        update_hash.insert(constants::ResourceType::Dog.str_key(), 1);

        let mut actions: Vec<Box<MoveAction>> = Vec::new();
        actions.push(Box::new(UpdateResources {
            player: game.next.clone(),
            update_hash,
        }));
        Actions {
            args: HashMap::new(),
            actions,
        }
    }
}

pub struct SlashAndBurn {}

impl Move for SlashAndBurn {
    fn get_name(&self) -> &str {
        MovesConstants::SLASH_AND_BURN
    }

    fn get_all_actions(&self, _game: Game, _moves_config: &MovesConfig) -> Vec<Actions> {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { args: HashMap::new(), actions });
        result
    }

    fn get_actions(&self, _game: Game, _moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        Actions {
            args: HashMap::new(),
            actions,
        }
    }
}

pub struct Blacksmithing {}

impl Move for Blacksmithing {
    fn get_name(&self) -> &str {
        MovesConstants::BLACKSMITHING
    }

    fn get_all_actions(&self, _game: Game, _moves_config: &MovesConfig) -> Vec<Actions> {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { args: HashMap::new(), actions });
        result
    }

    fn get_actions(&self, _game: Game, _moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        Actions {
            args: HashMap::new(),
            actions,
        }
    }
}

pub struct SheepFarming {}

impl Move for SheepFarming {
    fn get_name(&self) -> &str {
        MovesConstants::SHEEP_FARMING
    }

    fn get_all_actions(&self, _game: Game, _moves_config: &MovesConfig) -> Vec<Actions> {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { args: HashMap::new(), actions });
        result
    }

    fn get_actions(&self, _game: Game, _moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        Actions {
            args: HashMap::new(),
            actions,
        }
    }
}

pub struct OreMineConstruction {}

impl Move for OreMineConstruction {
    fn get_name(&self) -> &str {
        MovesConstants::ORE_MINE_CONSTRUCTION
    }

    fn get_all_actions(&self, _game: Game, _moves_config: &MovesConfig) -> Vec<Actions> {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { args: HashMap::new(), actions });
        result
    }

    fn get_actions(&self, _game: Game, _moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        Actions {
            args: HashMap::new(),
            actions,
        }
    }
}

pub struct WishForChildren {}

impl Move for WishForChildren {
    fn get_name(&self) -> &str {
        MovesConstants::WISH_FOR_CHILDREN
    }

    fn get_all_actions(&self, _game: Game, _moves_config: &MovesConfig) -> Vec<Actions> {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { args: HashMap::new(), actions });
        result
    }

    fn get_actions(&self, _game: Game, _moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        Actions {
            args: HashMap::new(),
            actions,
        }
    }
}

pub struct DonkeyFarming {}

impl Move for DonkeyFarming {
    fn get_name(&self) -> &str {
        MovesConstants::DONKEY_FARMING
    }

    fn get_all_actions(&self, _game: Game, _moves_config: &MovesConfig) -> Vec<Actions> {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { args: HashMap::new(), actions });
        result
    }

    fn get_actions(&self, _game: Game, _moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        Actions {
            args: HashMap::new(),
            actions,
        }
    }
}

pub struct RubyMineConstruction {}

impl Move for RubyMineConstruction {
    fn get_name(&self) -> &str {
        MovesConstants::RUBY_MINE_CONSTRUCTION
    }

    fn get_all_actions(&self, _game: Game, _moves_config: &MovesConfig) -> Vec<Actions> {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { args: HashMap::new(), actions });
        result
    }

    fn get_actions(&self, _game: Game, _moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        Actions {
            args: HashMap::new(),
            actions,
        }
    }
}

pub struct FamilyLife {}

impl Move for FamilyLife {
    fn get_name(&self) -> &str {
        MovesConstants::FAMILY_LIFE
    }

    fn get_all_actions(&self, _game: Game, _moves_config: &MovesConfig) -> Vec<Actions> {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { args: HashMap::new(), actions });
        result
    }

    fn get_actions(&self, _game: Game, _moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        Actions {
            args: HashMap::new(),
            actions,
        }
    }
}

pub struct OreDelivery {}

impl Move for OreDelivery {
    fn get_name(&self) -> &str {
        MovesConstants::ORE_DELIVERY
    }

    fn get_all_actions(&self, _game: Game, _moves_config: &MovesConfig) -> Vec<Actions> {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { args: HashMap::new(), actions });
        result
    }

    fn get_actions(&self, _game: Game, _moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        Actions {
            args: HashMap::new(),
            actions,
        }
    }
}

pub struct Adventure {}

impl Move for Adventure {
    fn get_name(&self) -> &str {
        MovesConstants::ADVENTURE
    }

    fn get_all_actions(&self, _game: Game, _moves_config: &MovesConfig) -> Vec<Actions> {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { args: HashMap::new(), actions });
        result
    }

    fn get_actions(&self, _game: Game, _moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        Actions {
            args: HashMap::new(),
            actions,
        }
    }
}

pub struct OreTrading {}

impl Move for OreTrading {
    fn get_name(&self) -> &str {
        MovesConstants::ORE_TRADING
    }

    fn get_all_actions(&self, _game: Game, _moves_config: &MovesConfig) -> Vec<Actions> {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { args: HashMap::new(), actions });
        result
    }

    fn get_actions(&self, _game: Game, _moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        Actions {
            args: HashMap::new(),
            actions,
        }
    }
}

pub struct RubyDelivery {}

impl Move for RubyDelivery {
    fn get_name(&self) -> &str {
        MovesConstants::RUBY_DELIVERY
    }

    fn get_all_actions(&self, _game: Game, _moves_config: &MovesConfig) -> Vec<Actions> {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        let mut result: Vec<Actions> = Vec::new();
        result.push(Actions { args: HashMap::new(), actions });
        result
    }

    fn get_actions(&self, _game: Game, _moves_config: &MovesConfig, _args: &HashMap<String, String>) -> Actions {
        let actions: Vec<Box<MoveAction>> = Vec::new();

        Actions {
            args: HashMap::new(),
            actions,
        }
    }
}
