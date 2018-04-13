use std::collections::{HashMap, HashSet};
use std::fs;
use std::path;
use serde_yaml;

use constants::{ResourceType, InsideElement, OutsideElement, GameStatus};
use config::{Config};
use rooms::constants::{GREEN_ROOMS, GINGER_ROOMS, YELLOW_ROOMS};
use rooms::core::{Room, get_from_string as get_room};
use models::moves::{MovesData};
use moves::core::{get_from_string as get_move, Move};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerCavern {
    pub cavern_type: InsideElement,
    pub position: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerRoom {
    pub room_type: String,
    pub position: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerField {
    pub field_type: OutsideElement,
    pub position: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub name: String,

    pub gnomes: u32,
    pub child_gnomes: u32,
    pub moved_gnomes: u32,

    pub fines: u32,

    pub caverns: Vec<PlayerCavern>,

    pub rooms: Vec<PlayerRoom>,

    pub fields: Vec<PlayerField>,

    pub resources: HashMap<String, u32>,

    pub moves: Vec<String>,

    pub warriors: Vec<u32>,
}

impl Player {
    pub fn change_resources(&mut self, delta: HashMap<String, u32>) {
        for (key, value) in delta.into_iter() {
            *self.resources.entry(key).or_insert(0) += value;
        }
    }

    pub fn spawn_new_gnome(&mut self) {
        self.child_gnomes += 1;
    }

    pub fn reserve_gnome(&mut self) {
        if self.gnomes >= self.moved_gnomes {
            panic!("All gnomes are moved");
        }
        self.moved_gnomes += 1;
    }

    pub fn add_rooms(&mut self, new_rooms: Vec<PlayerRoom>) {
        let mut slots: HashSet<u32> = HashSet::from(
            self.rooms.iter().map(|r| r.position).collect()
        );
        for room in new_rooms.iter() {
            if slots.contains(&room.position) {
                panic!(format!("Cannon add room {:?} to position {:?}", room.room_type, room.position));
            }
            slots.insert(room.position);
        }

        self.rooms.extend(new_rooms);
    }

    pub fn add_fields(&mut self, new_fields: Vec<PlayerField>) {
        let mut slots: HashSet<u32> = HashSet::from(
            self.fields.iter().map(|r| r.position).collect()
        );
        for field in new_fields.iter() {
            if slots.contains(&field.position) {
                panic!(format!("Cannon add field {:?} to position {:?}", field.field_type, field.position));
            }
            slots.insert(field.position);
        }

        self.fields.extend(new_fields);
    }

    pub fn get_rooms(&self) -> Vec<&Room> {
        self.rooms
            .iter()
            .map(|d| get_room(&d.room_type).unwrap())
            .collect()
    }

    pub fn get_all_gnomes_count(&self) -> u32 {
        self.gnomes + self.child_gnomes
    }

    pub fn get_free_gnomes(&self) -> u32 {
        self.gnomes - self.moved_gnomes
    }

    pub fn get_resource(&self, resource: ResourceType) -> u32 {
        *self.resources.get(&resource.str_key()).unwrap_or(&0)
    }

    pub fn get_free_room_slots(&self) -> u32 {
        let mut count = 0;
        for i in 0..16 {
            if self.rooms.iter().find(|r| r.position == i).is_none() {
                count += 1;
            }
        }
        count
    }

    pub fn get_free_field_slots(&self) -> u32 {
        let mut count = 0;
        for i in 0..16 {
            if self.fields.iter().find(|r| r.position == i).is_none() {
                count += 1;
            }
        }
        count
    }

    pub fn get_gnomes_slots(&self) -> u32 {
        self.rooms
            .iter()
            .map(|r| get_room(r.room_type.as_str()).unwrap().get_gnome_slots())
            .sum()
    }

    pub fn get_free_gnome_slots(&self) -> u32 {
        self.get_gnomes_slots() - self.gnomes - self.child_gnomes
    }

    pub fn get_yellow_rooms_count(&self) -> u32 {
        self.rooms.iter().filter(|r| YELLOW_ROOMS.contains(&r.room_type.as_str())).count() as u32
    }

    pub fn get_ginger_rooms_count(&self) -> u32 {
        self.rooms.iter().filter(|r| GINGER_ROOMS.contains(&r.room_type.as_str())).count() as u32
    }

    pub fn get_green_rooms_count(&self) -> u32 {
        self.rooms.iter().filter(|r| GREEN_ROOMS.contains(&r.room_type.as_str())).count() as u32
    }

    pub fn get_resource_clear_slots(&self, resource: &ResourceType) -> u32 {
        self.rooms
            .iter()
            .map(|r| get_room(&r.room_type).unwrap().get_slots().get_clear_slots(resource))
            .sum()
    }

    pub fn get_resource_max_slots(&self, resource: &ResourceType) -> u32 {
        self.rooms
            .iter()
            .map(|r| get_room(&r.room_type).unwrap().get_slots().get_max_slots(resource))
            .sum()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub turn: u64,

    pub status: GameStatus,

    pub next: String,
    pub first_move: String,
    pub order: Vec<String>,

    pub players: Vec<Player>,

    pub moves: MovesData,
    pub available_moves: Vec<String>,
}

impl Game {
    pub fn get_player(&self, player_name: &str) -> &Player {
        self.players
            .iter()
            .find(|p| p.name.eq(player_name))
            .unwrap()
    }

    pub fn get_player_mut(&mut self, player_name: &String) -> &mut Player {
        self.players
            .iter_mut()
            .find(|p| p.name == *player_name)
            .unwrap()
    }

    pub fn get_current_player_mut(&mut self) -> &mut Player {
        let current_player_name = &self.next;
        self.players
            .iter_mut()
            .find(|p| p.name == *current_player_name)
            .unwrap()
    }

    pub fn get_free_moves(&self) -> Vec<&Move> {
        self.available_moves
            .iter()
            .filter(|m| {
                for player in self.players.iter() {
                    if player.moves.contains(m) {
                        return false
                    }
                }
                true
            })
            .map(|m| get_move(&m).unwrap())
            .collect()
    }

    pub fn get_all_moves(&self) -> Vec<&Move> {
        self.available_moves
            .iter()
            .map(|m| get_move(&m).unwrap())
            .collect()
    }

    pub fn get_turn_moves_left(&self) -> u32 {
        self.players.iter().map(|p| p.get_free_gnomes()).sum()
    }

    pub fn is_last_move(&self) -> bool {
        self.get_turn_moves_left() <= 1
    }

    pub fn get_next_user(&self) -> String {
        let position = self.order.clone()
            .iter()
            .position(|p| *p == self.next)
            .unwrap() + 1;
        let before = self.order.clone().into_iter().take(position).collect::<Vec<_>>();
        let mut after = self.order.clone().into_iter().skip(position).collect::<Vec<_>>();
        after.extend(before);
        after
            .into_iter()
            .map(|n| self.get_player(&n).clone())
            .find(|p| p.get_free_gnomes() > 0)
            .unwrap()
            .name
    }

    pub fn get_last_game_file(config: &Config) -> (String, String) {
        let paths = fs::read_dir(&config.folder).unwrap();
        let mut max_file_number = 0_i64;
        for path in paths {
            let file_name = String::from(path.unwrap().file_name().clone().to_str().unwrap());
            let number: i64 = match file_name.trim_right_matches(".yml").parse() {
                   Ok(n) => {
                    n
                }
                Err(_) => {
                    -1
                }
            };
            if max_file_number < number {
                    max_file_number = number;
            }
        }
        (format!("{}.yml", max_file_number), format!("{}.yml", max_file_number + 1))
    }

    pub fn read_from_yaml(config: &Config, name: String) -> Game {
        let file = fs::File::open(path::Path::new(&config.folder).join(name))
            .expect("Error reading game file");
        serde_yaml::from_reader(file).unwrap()
    }

    pub fn write_to_yaml(&self, config: &Config, name: String) {
        let file = fs::File::create(path::Path::new(&config.folder).join(name))
            .expect("Error game file");
        serde_yaml::to_writer(file, &self).unwrap()
    }
}

pub trait GetPlayer<T> {
    fn get_player(&self, player_name: T) -> &Player;
}

impl<'a> GetPlayer<&'a str> for Game {
    fn get_player(&self, player_name: &str) -> &Player {
        self.players
            .iter()
            .find(|p| p.name.eq(player_name))
            .unwrap()
    }
}

impl<'a> GetPlayer<&'a String> for Game {
    fn get_player(&self, player_name: &String) -> &Player {
        self.players
            .iter()
            .find(|p| p.name.eq(player_name))
            .unwrap()
    }
}

