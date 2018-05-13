use actions::{
    MoveAction, Actions, NextUser, ReserveGnome, BlockMove, ChangeStatus, ReleaseMoves, OpenNewMove,
    SetFeedingAndBreedingStatus,
};
use constants::{GameStatus, FeedingAndBreedingStatus};
use models::game::{Game};


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
