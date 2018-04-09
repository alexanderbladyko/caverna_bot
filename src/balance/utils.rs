use std::collections::{HashMap};
use std::fs;
use serde_yaml;

use constants::{ALL_RESOURCES, TRIBAL_ANIMALS, ResourceType};
use actions::{constants as ActionsConstants, Actions, UpdateResources, BuildRooms};
use balance::{constants as BalanceConstants};
use rooms::{constants as RoomConstants};
use models::game::{Game, Player};
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

    pub fn read_from_yaml(path: String) -> BalanceConfig {
        let file = fs::File::open(path)
            .expect("Error reading game file");
        serde_yaml::from_reader(file).unwrap()
    }

    pub fn calculate(balance_item: &HashMap<String, f32>, game: &Game, player: &Player) -> f32 {
        let mut weight: f32 = 0f32;

        weight += (game.turn as f32) * *balance_item.get(&String::from(BalanceConstants::TURN)).unwrap();

        weight += (player.get_free_gnome_slots() as f32) * *balance_item.get(&String::from(BalanceConstants::FREE_GNOME_SLOTS_COUNT)).unwrap();

        weight += (player.get_free_room_slots() as f32) * *balance_item.get(&String::from(BalanceConstants::FREE_SLOTS_FOR_ROOM)).unwrap();
        weight += (player.get_free_gnome_slots() as f32) * *balance_item.get(&String::from(BalanceConstants::FREE_SLOTS_FOR_FIELD)).unwrap();

//        weight += (player.get_free_gnome_slots() as f32) * *balance_item.get(&String::from(BalanceConstants::FREE_SLOTS_FOR_CAVERNS)).unwrap();
//        weight += (player.get_free_gnome_slots() as f32) * *balance_item.get(&String::from(BalanceConstants::FREE_SLOTS_FOR_MINES)).unwrap();
//        weight += (player.get_free_gnome_slots() as f32) * *balance_item.get(&String::from(BalanceConstants::NEIGHBOURS_WITH_FIELDS)).unwrap();

        weight += (player.get_green_rooms_count() as f32) * *balance_item.get(&String::from(BalanceConstants::GREEN_ROOMS_COUNT)).unwrap();
        weight += (player.get_ginger_rooms_count() as f32) * *balance_item.get(&String::from(BalanceConstants::GINGER_ROOMS_COUNT)).unwrap();
        weight += (player.get_yellow_rooms_count() as f32) * *balance_item.get(&String::from(BalanceConstants::YELLOW_ROOMS_COUNT)).unwrap();

        weight += (player.get_all_gnomes_count() as f32) * *balance_item.get(&String::from(BalanceConstants::GNOMES_COUNT)).unwrap();
        weight += (*player.warriors.iter().max().unwrap_or(&0u32) as f32) * *balance_item.get(&String::from(BalanceConstants::MAX_WARRIOR_LEVEL)).unwrap();
        weight += (player.warriors.iter().count() as f32) * *balance_item.get(&String::from(BalanceConstants::WARRIOR_GNOMES_COUNT)).unwrap();
        weight += ((player.get_all_gnomes_count() - player.warriors.iter().count() as u32) as f32) * *balance_item.get(&String::from(BalanceConstants::PEACEFUL_GNOMES_COUNT)).unwrap();

        weight += (player.fines as f32) * *balance_item.get(&String::from(BalanceConstants::FINES_COUNT)).unwrap();

        weight += (player.get_all_gnomes_count() as f32) * *balance_item.get(&String::from(BalanceConstants::GNOMES_COUNT)).unwrap();

//        weight += (player.get_free_gnome_slots() as f32) * *balance_item.get(&String::from(BalanceConstants::FREE_ROOMS_COUNT)).unwrap();
//        weight += (player.get_free_gnome_slots() as f32) * *balance_item.get(&String::from(BalanceConstants::FREE_HALLS_COUNT)).unwrap();
//        weight += (player.get_free_gnome_slots() as f32) * *balance_item.get(&String::from(BalanceConstants::FREE_MINE_HALLS_COUNT)).unwrap();
//        weight += (player.get_free_gnome_slots() as f32) * *balance_item.get(&String::from(BalanceConstants::FREE_FIELDS_COUNT)).unwrap();

        ALL_RESOURCES.iter().for_each(|r| {
            weight += (*player.resources.get(&r.str_key()).unwrap_or(&0) as f32) * *balance_item.get(&resource_count(&r)).unwrap();

        });

        TRIBAL_ANIMALS.iter().for_each(|r| {
            weight += (player.get_resource_clear_slots(r) as f32) * *balance_item.get(&clear_resource_slots_count(&r)).unwrap();
            weight += (player.get_resource_max_slots(r) as f32) * *balance_item.get(&max_resource_slots_count(&r)).unwrap();
        });

        weight
    }
}

pub fn resource_count(resource: &ResourceType) -> String{
    format!("{}{}", BalanceConstants::RESOURCE, &resource.str_key())
}

pub fn clear_resource_slots_count(resource: &ResourceType) -> String{
    format!("{}{}", BalanceConstants::CLEAR_SLOTS_FOR, &resource.str_key())
}

pub fn max_resource_slots_count(resource: &ResourceType) -> String{
    format!("{}{}", BalanceConstants::MAX_SLOTS_FOR, &resource.str_key())
}


pub fn generate_balance_item() -> HashMap<String, f32> {
    let mut map: HashMap<String, f32> = hash_map! {
        String::from(BalanceConstants::TURN) => 0f32,

        String::from(BalanceConstants::FREE_GNOME_SLOTS_COUNT) => 0f32,

        String::from(BalanceConstants::FREE_SLOTS_FOR_ROOM) => 0f32,
        String::from(BalanceConstants::FREE_SLOTS_FOR_FIELD) => 0f32,
        String::from(BalanceConstants::FREE_SLOTS_FOR_CAVERNS) => 0f32,
        String::from(BalanceConstants::FREE_SLOTS_FOR_MINES) => 0f32,

        String::from(BalanceConstants::NEIGHBOURS_WITH_FIELDS) => 0f32,

        String::from(BalanceConstants::GREEN_ROOMS_COUNT) => 0f32,
        String::from(BalanceConstants::YELLOW_ROOMS_COUNT) => 0f32,
        String::from(BalanceConstants::GINGER_ROOMS_COUNT) => 0f32,

        String::from(BalanceConstants::GNOMES_COUNT) => 0f32,
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
            resource_count(&resource) => 0f32
        })
    }
    for resource in TRIBAL_ANIMALS.iter() {
        hash_map!(map, {
            max_resource_slots_count(&resource) => 0f32,
            clear_resource_slots_count(&resource) => 0f32
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

pub fn get_balance_weight(game: &Game, player_name: &str, balance_config: &BalanceConfig, actions: &Actions) -> i32 {
    let mut weight: f32 = 0f32;

    let player = game.get_player(player_name);

    actions.actions.iter().for_each(|action| {
        if !ActionsConstants::ALL_PLAYER_ACTIONS.contains(&action.get_name()) {
            return
        }
        match action.get_name() {
            ActionsConstants::UPDATE_RESOURCES => {
                let resources_update: &UpdateResources = action.as_any().downcast_ref::<UpdateResources>().unwrap();
                resources_update.update_hash.iter().for_each(|(resource, count)| {
                    let score = BalanceConfig::calculate(balance_config.resources.get(resource).unwrap(), game, player);
                    weight += (*count as f32) * score;
                });
            },
            ActionsConstants::BUILD_ROOMS => {
                let build_rooms: &BuildRooms = action.as_any().downcast_ref::<BuildRooms>().unwrap();
                build_rooms.rooms.iter().for_each(|room| {
                    let score = BalanceConfig::calculate(balance_config.rooms.get(&room.room_type).unwrap(), game, player);
                    weight += score;
                });
            },
            _ => {
                let action_balance = balance_config.actions.get(action.get_name()).unwrap();
                weight += BalanceConfig::calculate(action_balance, game, player);
            }
        }
    });


    weight.round() as i32
}