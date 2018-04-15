use actions::{
    MoveAction, Actions, NextUser, ReserveGnome, BlockMove, ChangeStatus, ReleaseMoves,
};
use constants::GameStatus;
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


pub fn get_game_turn_actions(game: &Game) -> Actions {
    Actions::from_vec(vec![
        Box::from(ChangeStatus {
            status: GameStatus::PlayerMove,
        }),
        Box::from(ReleaseMoves {}),
        Box::from(NextUser {
            player: (*game.order.first().unwrap()).clone(),
        })
    ])
}


pub fn get_start_feeding_actions(game: &Game) -> Actions {
    Actions::from_vec(vec![
        Box::from(ChangeStatus {
            status: GameStatus::GnomeFeeding,
        }),
        Box::from(NextUser {
            player: (*game.order.first().unwrap()).clone(),
        })
    ])
}

pub fn get_tribal_breeding_actions(game: &Game) -> Actions {
    Actions::from_vec(vec![
        Box::from(ChangeStatus {
            status: GameStatus::TribalBreeding,
        }),
        Box::from(NextUser {
            player: (*game.order.first().unwrap()).clone(),
        })
    ])
}
