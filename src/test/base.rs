use std::collections::HashMap;
use constants;
use models::game::{Game, Player};
use models::moves;
use moves::{constants as MovesConstants};
use moves::{config as MovesConfig};

pub fn get_game_with_2_players() -> Game {
    Game {
        turn: 1,

        status: constants::GameStatus::PlayerMove,
        next: String::from("p1"),
        first_move: String::from("p1"),
        order: vec![String::from("p1"), String::from("p2")],

        feeding_and_breeding_status: constants::FeedingAndBreedingStatus::Normal,

        players: vec![
            Player {
                name: String::from("p1"),

                gnomes: 2,
                child_gnomes: 0,
                moved_gnomes: 0,

                caverns: vec![],

                fields: vec![],

                rooms: vec![],

                resources: HashMap::new(),

                moves: vec![],

                fines: 0,

                warriors: vec![],
            },
            Player {
                name: String::from("p2"),

                gnomes: 2,
                child_gnomes: 0,
                moved_gnomes: 0,

                caverns: vec![],

                rooms: vec![],

                fields: vec![],

                resources: HashMap::new(),

                moves: vec![],

                fines: 0,

                warriors: vec![],
            },
        ],

        moves: moves::MovesData {
            drift_mining: moves::DriftMiningData {
                stone: 1,
            },
            logging: moves::LoggingData {
                wood: 1,
            },
            wood_gathering: moves::WoodGatheringData {
                wood: 1,
            },
            excavation: moves::ExcavationData {
                stone: 1,
            },
            clearing: moves::ClearingData {
                wood: 1,
            },
            starting_player: moves::StartingPlayerData {
                food: 1,
            },
            ruby_mining: moves::RubyMiningData {
                gems: 0,
            },
        },
        available_moves: vec![
            String::from(MovesConstants::DRIFT_MINING),
            String::from(MovesConstants::LOGGING),
            String::from(MovesConstants::WOOD_GATHERING),
            String::from(MovesConstants::EXCAVATION),
            String::from(MovesConstants::SUPPLIES),
            String::from(MovesConstants::CLEARING),
            String::from(MovesConstants::STARTING_PLAYER),
        ],
    }
}


pub fn get_moves_config() -> MovesConfig::MovesConfig {
    MovesConfig::MovesConfig {
        drift_mining: MovesConfig::DriftMining {
            stone_incr: 1,
        },
        logging: MovesConfig::Logging {
            secondary_wood_incr: 1,
            wood_incr: 1,
        },
        wood_gathering: MovesConfig::WoodGathering {
            wood_incr: 1,
        },
        excavation: MovesConfig::Excavation {
            stone_incr: 1,
            secondary_stone_incr: 1,
        },
        supplies: MovesConfig::Supplies {
            coal: 1,
            food: 1,
            gold: 1,
            stone: 1,
            wood: 1,
        },
        clearing: MovesConfig::Clearing {
            wood_incr: 1,
        },
        starting_player: MovesConfig::StartingPlayer {
            coal: 1,
            food_incr: 1,
            gem: 1,
        },
        ruby_mining: MovesConfig::RubyMining {
            gems: 1,
            gem_incr: 1,
            from_turn: 2,
        },
    }
}


pub fn assert_player_has_resource(game: &Game, player_name: String, resource: constants::ResourceType, value: u32) {
    let player = game.players.iter().find(|p| p.name == player_name).unwrap();
    assert_eq!(*player.resources.get(&resource.str_key()).unwrap(), value);
}