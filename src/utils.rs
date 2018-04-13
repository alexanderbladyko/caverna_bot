use std::collections::HashMap;

use actions::{MoveAction, Actions, NextUser, ReserveGnome, BlockMove, ChangeStatus};
use constants::GameStatus;
use models::game::{Game};


pub fn get_player_move_actions(move_name: String, game: &Game) -> Actions {
    let mut actions: Vec<Box<MoveAction>> = vec![
        Box::from(ReserveGnome {
            player: game.next.clone(),
        }),
        Box::from(BlockMove {
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

    Actions {
        args: HashMap::new(),
        actions,
    }
}
