use std::collections::HashMap;
use constants;
use models::game::{Game, Player};
use models::moves;

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
        },
        available_moves: vec![
            String::from("drift_mining"),
            String::from("logging"),
            String::from("wood_gathering"),
            String::from("excavation"),
            String::from("supplies"),
            String::from("clearing"),
            String::from("starting_player"),
        ],
    }
}


pub fn assert_player_has_resource(game: &Game, player_name: String, resource: constants::ResourceType, value: u32) {
    let player = game.players.iter().find(|p| p.name == player_name).unwrap();
    assert_eq!(*player.resources.get(&resource.str_key()).unwrap(), value);
}