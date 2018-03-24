use std::collections::{HashMap};
use std::fs;
use serde_yaml;

use constants::{ALL_RESOURCES, TRIBAL_ANIMALS};
use actions::{constants as ActionsConstants, Actions};
use balance::{constants as BalanceConstants};
use rooms::{constants as RoomConstants};
use models::game::Game;
use moves::{constants as MovesConstants};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BalanceConfig {
    pub actions: HashMap<String, HashMap<String, f32>>,
    pub resources: HashMap<String, HashMap<String, f32>>,
    pub rooms: HashMap<String, HashMap<String, f32>>,
}

impl BalanceConfig {
    pub fn write_to_yaml(&self, path: String) {
        let file = fs::File::create(path).expect("Error game file");
        serde_yaml::to_writer(file, &self).unwrap()
    }
}

pub fn generate_balance_item() -> HashMap<String, f32> {
    let mut map: HashMap<String, f32> = hash_map! {
        String::from(BalanceConstants::FREE_GNOME_SLOTS_COUNT) => 0f32,
        String::from(BalanceConstants::TURN) => 0f32,
        String::from(BalanceConstants::FREE_SLOTS_FOR_ROOM) => 0f32,
        String::from(BalanceConstants::FREE_SLOTS_FOR_FIELD) => 0f32,
        String::from(BalanceConstants::FREE_SLOTS_FOR_CAVERNS) => 0f32,
        String::from(BalanceConstants::FREE_SLOTS_FOR_MINES) => 0f32,
        String::from(BalanceConstants::NEIGHBOURS_WITH_FIELDS) => 0f32,
        String::from(BalanceConstants::GREEN_ROOMS_COUNT) => 0f32,
        String::from(BalanceConstants::YELLOW_ROOMS_COUNT) => 0f32,
        String::from(BalanceConstants::GINGER_ROOMS_COUNT) => 0f32,
        String::from(BalanceConstants::WARRIOR_GNOMES_COUNT) => 0f32,
        String::from(BalanceConstants::PEACEFUL_GNOMES_COUNT) => 0f32,
        String::from(BalanceConstants::MAX_WARRIOR_LEVEL) => 0f32,
        String::from(BalanceConstants::FINES_COUNT) => 0f32,
        String::from(BalanceConstants::FREE_ROOMS_COUNT) => 0f32,
        String::from(BalanceConstants::FREE_HALLS_COUNT) => 0f32,
        String::from(BalanceConstants::FREE_MINE_HALLS_COUNT) => 0f32,
        String::from(BalanceConstants::FREE_FIELDS_COUNT) => 0f32
    };
    for resource in ALL_RESOURCES.iter() {
        hash_map!(map, {
            format!("{}{}", BalanceConstants::RESOURCE, &resource.clone().str_key()) => 0f32
        })
    }
    for resource in TRIBAL_ANIMALS.iter() {
        hash_map!(map, {
            format!("{}{}", BalanceConstants::MAX_SLOTS_FOR, &resource.clone().str_key()) => 0f32,
            format!("{}{}", BalanceConstants::CLEAR_SLOTS_FOR, &resource.clone().str_key()) => 0f32
        })
    }
    map
}

pub fn generate_room_with_items() -> HashMap<String, HashMap<String, f32>> {
    let mut hash: HashMap<String, HashMap<String, f32>> = HashMap::new();
    RoomConstants::ALL_ROOMS.iter().for_each(|r| {
        hash.insert(String::from(*r), generate_balance_item());
    });
    hash
}

pub fn generate_resources_with_items() -> HashMap<String, HashMap<String, f32>> {
    let mut hash: HashMap<String, HashMap<String, f32>> = HashMap::new();
    ALL_RESOURCES.into_iter().for_each(|r| {
        hash.insert(r.clone().str_key(), generate_balance_item());
    });
    hash
}

pub fn generate_moves_with_items() -> HashMap<String, HashMap<String, f32>> {
    let mut hash: HashMap<String, HashMap<String, f32>> = HashMap::new();
    MovesConstants::TWO_PLAYERS_MOVES.into_iter().for_each(|r| {
        hash.insert(String::from(*r), generate_balance_item());
    });
    hash
}

pub fn generate_actions_with_items() -> HashMap<String, HashMap<String, f32>> {
    let mut hash: HashMap<String, HashMap<String, f32>> = HashMap::new();
    ActionsConstants::ALL_PLAYER_ACTIONS.into_iter().for_each(|r| {
        if *r != ActionsConstants::UPDATE_RESOURCES {
            hash.insert(String::from(*r), generate_balance_item());
        }
    });
    hash
}

pub fn generate_balance_config() -> BalanceConfig {
    BalanceConfig {
        actions: generate_actions_with_items(),
        rooms: generate_room_with_items(),
        resources: generate_resources_with_items(),
    }
}

pub fn get_balance_weight(game: &Game, player: &str, _balance_config: &BalanceConfig, _actions: &Actions) -> i32 {
    let weight = 0;

    let _player = game.get_player(player);

    weight
}