use std::collections::HashMap;
use constants;
use balance::utils::{BalanceConfig, get_balance_weight};
use models::game::{Game, Player};
use models::moves;
use moves::config::{MovesConfig};
use moves::core::{collect_actions};

pub fn simulate_2_players_game(moves_config: &MovesConfig, config1: &BalanceConfig, config2: &BalanceConfig) {
    let mut game = _instantiate_game();
    let balances = hash_map! {
        String::from("p1") => config1,
        String::from("p2") => config2
    };

    _run_one_round(&mut game, moves_config, balances);

}

fn _run_one_round(game: &mut Game, moves_config: &MovesConfig, configs: HashMap<String, &BalanceConfig>) {
    while game.is_last_move() == false {
        let game_cloned = game.clone();
        let player = game_cloned.get_next_user();
        let balance_config = configs.get(&player).unwrap();
        let moves = game_cloned.get_free_moves();
        let actions = collect_actions(game, moves_config, moves);
        let max_actions = actions
            .iter()
            .max_by_key(|a| get_balance_weight(&game_cloned, player.as_str(), balance_config, a))
            .unwrap();
        max_actions.perform(game);
    }
}

fn _instantiate_game() -> Game {
    Game {
        turn: 1,
        status: constants::GameStatus::GnomeFeeding,
        next: String::from("p1"),
        first_move: String::from("p1"),
        order: vec![String::from("p1"), String::from("p2")],
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