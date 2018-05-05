use std::collections::HashMap;

use actions::{MoveAction, Actions};
use constants::{FeedingAndBreedingStatus};
use models::game::{Player};


pub fn get_feeding_and_breeding_actions(_player: &Player, _feeding_and_breeding_status: FeedingAndBreedingStatus) -> Vec<Actions> {
    let actions: Vec<Box<MoveAction>> = Vec::new();

    let mut result: Vec<Actions> = Vec::new();
    result.push(Actions {
        args: hash_map! {
                String::from("hall_slot") => String::new(),
                String::from("room_slot") => String::new()
            },
        actions,
    });
    result
}