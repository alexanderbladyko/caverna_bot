use actions::{
    MoveAction, Actions, NextUser, ReserveGnome, BlockMove, ChangeStatus, ReleaseMoves, OpenNewMove,
    SetFeedingAndBreedingStatus,
};
use constants::{GameStatus, FeedingAndBreedingStatus};
use models::game::{Game};
use std::collections::{HashSet};


pub fn get_player_move_actions(move_name: String, game: &Game) -> Actions {
    let mut actions: Vec<Box<MoveAction>> = vec![
        Box::from(ReserveGnome {
            player: game.next.clone(),
        }),
        Box::from(BlockMove {
            player: game.next.clone(),
            player_move: move_name,
        }),
        Box::from(NextUser {
            player: game.get_next_user(),
        })
    ];

    if game.is_last_move() {
        actions.push(Box::from(ChangeStatus {
            status: GameStatus::NextTurnPending
        }));
    }

    Actions::from_vec(actions)
}


pub fn get_game_turn_actions(game: &Game, new_move: Option<&str>) -> Actions {
    let mut actions: Vec<Box<MoveAction>> = vec![
        Box::from(ChangeStatus {
            status: GameStatus::PlayerMove,
        }),
        Box::from(ReleaseMoves {}),
        Box::from(NextUser {
            player: (*game.order.first().unwrap()).clone(),
        }),
    ];
    match new_move {
        Some(m) => actions.push(Box::from(OpenNewMove {
            new_move: String::from(m),
        })),
        None => (),
    };
    Actions::from_vec(actions)
}

pub fn get_start_feeding_and_breeding_actions(game: &Game, status: FeedingAndBreedingStatus) -> Actions {
    Actions::from_vec(vec![
        Box::from(ChangeStatus {
            status: GameStatus::FeedingAndBreeding,
        }),
        Box::from(NextUser {
            player: (*game.order.first().unwrap()).clone(),
        }),
        Box::from(SetFeedingAndBreedingStatus {
            status,
        }),
    ])
}

pub fn get_available_slots(reserved_slots: Vec<u32>) -> HashSet<u32> {
    let mut result: HashSet<u32> = HashSet::new();
    for i in 0..3 {
        for j in 0..4 {
            let num = i + 3 * j;
            if reserved_slots.contains(&num) {
                continue;
            }
            if i > 0 && reserved_slots.contains(&(num - 1)) {
                result.insert(num);
            }
            if i < 2 && reserved_slots.contains(&(num + 1)) {
                result.insert(num);
            }
            if j > 0 && reserved_slots.contains(&(num - 3)) {
                result.insert(num);
            }
            if j < 3 && reserved_slots.contains(&(num + 3)) {
                result.insert(num);
            }
        }
    }
    result
}

pub fn get_available_pair_slots(reserved_slots: Vec<u32>) -> HashSet<(u32, u32)> {
    let mut result: HashSet<(u32, u32)> = HashSet::new();
    let available_slots = get_available_slots(reserved_slots.clone());
    for i in 0..3 {
        for j in 0..4 {
            let num = i + 3 * j;
            if reserved_slots.contains(&num) {
                continue;
            }
            if i > 0 && available_slots.contains(&(num - 1)) {
                result.insert((num - 1, num));
            }
            if i < 2 && available_slots.contains(&(num + 1)) {
                result.insert((num, num + 1));
            }
            if j > 0 && available_slots.contains(&(num - 3)) {
                result.insert((num - 3, num));
            }
            if j < 3 && available_slots.contains(&(num + 3)) {
                result.insert((num, num + 3));
            }
        }
    }
    result
}