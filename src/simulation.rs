use std::collections::HashMap;
use rand::{random};

use balance::utils::{BalanceConfig, get_balance_weight};
use constants;
use models::game::{Game, Player, PlayerRoom};
use models::moves;
use moves::config::{MovesConfig};
use moves::core::{collect_actions};
use moves::{constants as MovesConstants};
use moves::feeding::get_feeding_and_breeding_actions;
use rooms::constants::ENTRY_LEVEL_DWELLING;
use score::calculator::get_final_score;
use utils::{get_player_move_actions, get_game_turn_actions, get_start_feeding_and_breeding_actions};


pub fn mutate_config(base: &BalanceConfig) -> BalanceConfig {
    let mut config = base.clone();

    for _ in 0..3 {
        match random::<i32>() % 3 {
            0 => _mutate_hash(&mut config.actions),
            1 => _mutate_hash(&mut config.rooms),
            2 => _mutate_hash(&mut config.resources),
            _ => panic!(),
        }
    }

    config
}

pub fn _mutate_hash(cfg: &mut HashMap<String, HashMap<String, f32>>) {
    let first = random::<usize>() % cfg.len();
    let some_key = cfg.keys().into_iter().skip(first).next().unwrap().clone();

    let inner_hash = cfg.get_mut(&some_key).unwrap();

    let second = random::<usize>() % inner_hash.len();
    let some_inner_key = inner_hash.keys().skip(second).next().unwrap().clone();

    let delta: f32 = ((2.0 * random::<f32>() - 1.0) * 10f32).round() / 10f32;
    *inner_hash.get_mut(&some_inner_key).unwrap() += delta;
}

pub fn simulate_tournament(moves_config: &MovesConfig, configs: Vec<BalanceConfig>) -> (i32, i32) {
    let mut score_table: Vec<i32> = vec![0; configs.len()];
    let mut sum_score_table: Vec<i32> = vec![0; configs.len()];

    for i in 0..configs.len()-1 {
        for j in i+1..configs.len() {
            let (first_score, second_score) = simulate_2_players_game(moves_config, &configs[i], &configs[j]);
            sum_score_table[i] += first_score;
            sum_score_table[j] += second_score;
            if first_score == second_score {
                score_table[i] += 1;
                score_table[j] += 1;
            } else if first_score > second_score {
                score_table[i] += 3;
            } else {
                score_table[j] += 3;
            }
        }
    }
    let (max, _) = score_table
        .iter()
        .enumerate()
        .max_by_key(|&(_, item)| item)
        .unwrap();
    let (max_by_score, _) = sum_score_table
        .iter()
        .filter(|s| **s != max as i32)
        .enumerate()
        .max_by_key(|&(_, item)| item)
        .unwrap();
    (max as i32, max_by_score as i32)
}

pub fn simulate_2_players_game(moves_config: &MovesConfig, config1: &BalanceConfig, config2: &BalanceConfig) -> (i32, i32) {
    let mut game = _instantiate_game();
    let balances = hash_map! {
        String::from("p1") => config1,
        String::from("p2") => config2
    };

    // Round #1
    _run_one_round(&mut game, moves_config, &balances);
    _run_finish_round(&mut game, moves_config, Option::from(MovesConstants::SHEEP_FARMING));

    // Round #2
    _run_one_round(&mut game, moves_config, &balances);
    _run_finish_round(&mut game, moves_config, Option::from(MovesConstants::BLACKSMITHING));

    // Round #3
    _run_one_round(&mut game, moves_config, &balances);
    _run_feed_and_breed_round(&mut game, constants::FeedingAndBreedingStatus::Normal, &balances);
    _run_finish_round(&mut game, moves_config, Option::from(MovesConstants::ORE_MINE_CONSTRUCTION));

    // Round #4
    _run_one_round(&mut game, moves_config, &balances);
    _run_feed_and_breed_round(&mut game, constants::FeedingAndBreedingStatus::FeedByOne, &balances);
    _run_finish_round(&mut game, moves_config, Option::from(MovesConstants::WISH_FOR_CHILDREN));

    // Round #5
    _run_one_round(&mut game, moves_config, &balances);
    _run_feed_and_breed_round(&mut game, constants::FeedingAndBreedingStatus::Normal, &balances);
    _run_finish_round(&mut game, moves_config, Option::from(MovesConstants::DONKEY_FARMING));

    // Round #6
    _run_one_round(&mut game, moves_config, &balances);
    _run_feed_and_breed_round(&mut game, constants::FeedingAndBreedingStatus::Normal, &balances);
    _run_finish_round(&mut game, moves_config, Option::from(MovesConstants::RUBY_MINE_CONSTRUCTION));

    // Round #7
    _run_one_round(&mut game, moves_config, &balances);
    _run_feed_and_breed_round(&mut game, constants::FeedingAndBreedingStatus::NoBreeding, &balances);
    _run_finish_round(&mut game, moves_config, Option::from(MovesConstants::ORE_DELIVERY));

    // Round #8
    _run_one_round(&mut game, moves_config, &balances);
    _run_feed_and_breed_round(&mut game, constants::FeedingAndBreedingStatus::FeedByOne, &balances);
    _run_finish_round(&mut game, moves_config, Option::from(MovesConstants::FAMILY_LIFE));

    // Round #9 is skipped for 2 players

    // Round #10
    _run_one_round(&mut game, moves_config, &balances);
    _run_feed_and_breed_round(&mut game, constants::FeedingAndBreedingStatus::Normal, &balances);
    _run_finish_round(&mut game, moves_config, Option::from(MovesConstants::ADVENTURE));

    // Round #11
    _run_one_round(&mut game, moves_config, &balances);
    _run_feed_and_breed_round(&mut game, constants::FeedingAndBreedingStatus::FeedingOrBreeding, &balances);
    _run_finish_round(&mut game, moves_config, Option::from(MovesConstants::RUBY_DELIVERY));

    // Round #12
    _run_one_round(&mut game, moves_config, &balances);
    _run_feed_and_breed_round(&mut game, constants::FeedingAndBreedingStatus::Normal, &balances);
    _run_finish_round(&mut game, moves_config, Option::from(MovesConstants::ORE_TRADING));

    (get_final_score(game.clone(), &String::from("p1")), get_final_score(game.clone(), &String::from("p2")))
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

        let move_actions = get_player_move_actions(max_actions.move_name.clone(), game);
        move_actions.perform(game);
    }
}

fn _run_feed_and_breed_round(game: &mut Game, status: constants::FeedingAndBreedingStatus, configs: &HashMap<String, &BalanceConfig>) {
    let actions = get_start_feeding_and_breeding_actions(game, status.clone());
    actions.perform(game);

    let players = game.players.clone();

    players.into_iter().for_each(|player| {
        let game_cloned = game.clone();

        let balance_config = configs.get(&player.name).unwrap();

        let feeding_actions = get_feeding_and_breeding_actions(&player, status.clone());

        let max_actions = feeding_actions
            .iter()
            .max_by_key(|a| get_balance_weight(&game_cloned, player.name.as_str(), balance_config, &a))
            .unwrap();
        max_actions.perform(game);
    })

}

fn _run_finish_round(game: &mut Game, moves_config: &MovesConfig, new_move: Option<&str>) {
    let actions = get_game_turn_actions(game, new_move);
    actions.perform(game);

    for mov in game.clone().get_all_moves() {
        mov.on_next_turn(game, &moves_config);
    }
}

fn _instantiate_game() -> Game {
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
            ruby_mining: moves::RubyMiningData {
                gems: 0,
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