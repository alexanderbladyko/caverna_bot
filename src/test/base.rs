use std::collections::HashMap;
use constants;
use models::game::{Game, Player};
use models::moves;

pub fn get_game_with_2_players() -> Game {
    Game {
        turn: 1,

        next: String::from("p1"),
        first_move: String::from("p1"),
        order: vec![String::from("p1"), String::from("p2")],

        players: vec![
            Player {
                name: String::from("p1"),

                gnomes: 2,
                child_gnomes: 0,
                moved_gnomes: 0,

                rooms: vec![],
                room_slots_count: 12,

                fields: vec![],
                field_slots_count: 12,

                dwellings: vec![],

                resources: HashMap::new(),

                moves: vec![],
            },
            Player {
                name: String::from("p2"),

                gnomes: 2,
                child_gnomes: 0,
                moved_gnomes: 0,

                rooms: vec![],
                room_slots_count: 12,

                dwellings: vec![],

                fields: vec![],
                field_slots_count: 12,

                resources: HashMap::new(),

                moves: vec![],

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