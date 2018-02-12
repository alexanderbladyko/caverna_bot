use std::collections::{HashMap};
use std::fs;
use serde_yaml;

use constants::{ResourceType, ALL_RESOURCES};
use balance::{constants};
use rooms::{constants as RoomConstants};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BalanceConfig {
    pub moves: HashMap<String, HashMap<String, f32>>,
    pub resources: HashMap<String, HashMap<String, f32>>,
}

impl BalanceConfig {
    pub fn write_to_yaml(&self, path: String) {
        let file = fs::File::create(path).expect("Error game file");
        serde_yaml::to_writer(file, &self).unwrap()
    }
}

pub fn generate_balance_item() -> HashMap<String, f32> {
    let mut map: HashMap<String, f32> = hash_map! {
        String::from(constants::FREE_GNOME_SLOTS_COUNT) => 0f32,
        String::from(constants::TURN) => 0f32,
        String::from(constants::FREE_SLOTS_FOR_ROOM) => 0f32,
        String::from(constants::FREE_SLOTS_FOR_FIELD) => 0f32,
        String::from(constants::FREE_SLOTS_FOR_CAVERNS) => 0f32,
        String::from(constants::FREE_SLOTS_FOR_MINES) => 0f32,
        String::from(constants::NEIGHBOURS_WITH_FIELDS) => 0f32,
        String::from(constants::GREEN_ROOMS_COUNT) => 0f32,
        String::from(constants::YELLOW_ROOMS_COUNT) => 0f32,
        String::from(constants::GINGER_ROOMS_COUNT) => 0f32,
        String::from(constants::WARRIOR_GNOMES_COUNT) => 0f32,
        String::from(constants::PEACEFUL_GNOMES_COUNT) => 0f32,
        String::from(constants::MAX_WARRIOR_LEVEL) => 0f32,
        String::from(constants::FINES_COUNT) => 0f32,
        String::from(constants::FREE_ROOMS_COUNT) => 0f32,
        String::from(constants::FREE_HALLS_COUNT) => 0f32,
        String::from(constants::FREE_MINE_HALLS_COUNT) => 0f32,
        String::from(constants::FREE_FIELDS_COUNT) => 0f32
    };
    for resource in ALL_RESOURCES.iter() {
        hash_map!(map, {
            format!("{}{}", constants::MAX_SLOTS_FOR, &resource.clone().str_key()) => 0f32,
            format!("{}{}", constants::CLEAR_SLOTS_FOR, &resource.clone().str_key()) => 0f32,
            format!("{}{}", constants::RESOURCE, &resource.clone().str_key()) => 0f32
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

pub fn generate_balance_config() -> BalanceConfig {
    BalanceConfig {
        moves: generate_room_with_items(),
        resources: generate_resources_with_items(),
    }
}