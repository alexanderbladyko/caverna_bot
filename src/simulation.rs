use std::collections::HashMap;
use balance::utils::{BalanceConfig, get_balance_weight};
use constants;
use models::game::{Game, Player, PlayerRoom};
use models::moves;
use moves::config::{MovesConfig};
use moves::core::{collect_actions};
use rooms::constants::ENTRY_LEVEL_DWELLING;
use utils::{get_player_move_actions, get_game_turn_actions, get_start_feeding_actions, get_tribal_breeding_actions};

pub fn simulate_2_players_game(moves_config: &MovesConfig, config1: &BalanceConfig, config2: &BalanceConfig) {
    let mut game = _instantiate_game();
    let balances = hash_map! {
        String::from("p1") => config1,
        String::from("p2") => config2
    };

    _run_one_round(&mut game, moves_config, &balances);
    _run_finish_round(&mut game, moves_config);

    println!("{:?}", game);
    println!("——————-");

    _run_one_round(&mut game, moves_config, &balances);
    _run_finish_round(&mut game, moves_config);

    println!("{:?}", game);
    println!("——————-");

}

fn _run_one_round(game: &mut Game, moves_config: &MovesConfig, configs: &HashMap<String, &BalanceConfig>) {
    while game.get_turn_moves_left() != 0 {
        let game_cloned = game.clone();
        let player = game_cloned.get_next_user();
        let balance_config = configs.get(&player).unwrap();
        let moves = game_cloned.get_free_moves();
        let actions = collect_actions(game, moves_config, moves);

        let max_actions = actions
            .iter()
            .max_by_key(|a| get_balance_weight(&game_cloned, player.as_str(), balance_config, &a.actions))
            .unwrap();
        max_actions.actions.perform(game);
        println!("{:?}", max_actions.move_name);
        println!("{:?}", max_actions.actions.get_info());

        let move_actions = get_player_move_actions(max_actions.move_name.clone(), game);
        move_actions.perform(game);
        println!("{:?}", game);
        println!("——————-");
    }
}

fn _run_feed_round(game: &mut Game) {
    let actions = get_start_feeding_actions(game);
    actions.perform(game);
}

fn _run_breed_round(game: &mut Game) {
    let actions = get_tribal_breeding_actions(game);
    actions.perform(game);
}

fn _run_finish_round(game: &mut Game, moves_config: &MovesConfig) {
    let actions = get_game_turn_actions(game);
    actions.perform(game);

    for mov in game.clone().get_all_moves() {
        mov.on_next_turn(game, &moves_config);
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
                rooms: vec![
                    PlayerRoom {
                        position: 0u32,
                        room_type: String::from(ENTRY_LEVEL_DWELLING),
                    }
                ],
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
                rooms: vec![
                    PlayerRoom {
                        position: 0u32,
                        room_type: String::from(ENTRY_LEVEL_DWELLING),
                    }
                ],
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