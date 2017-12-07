use moves::actions::{MoveAction};

use models::game::{Game};


pub struct Actions {
    weight: i64,
    actions: Vec<Box<MoveAction>>,
}

pub trait Move {
    fn get_actions(self, game: Game) -> Actions;
}