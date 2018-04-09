use actions::{NextUser, ReserveGnome, BlockMove, MoveAction};
use models::game::{Game};

pub fn get_player_move_action(action_name: String, game: &Game) -> Vec<Box<MoveAction>> {
    vec![
        Box::from(ReserveGnome {
            player: game.next.clone(),
        }),
        Box::from(BlockMove {
            player_move: action_name,
        }),
        Box::from(NextUser {
            player: game.get_next_user(),
        })
    ]
}