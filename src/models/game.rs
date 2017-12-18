use std::collections::{HashMap, HashSet};
use std::fs;
use std::path;
use serde_yaml;

use constants::{InsideElement, OutsideElement};
use config::{Config};
use models::moves::{MovesData};
use moves::core::{get_from_string, Move};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerRoom {
    pub room_type: InsideElement,
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

    pub rooms: Vec<PlayerRoom>,
    pub room_slots_count: u32,

    pub fields: Vec<PlayerField>,
    pub field_slots_count: u32,

    pub resources: HashMap<String, u32>,

    pub moves: Vec<String>,
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub turn: u64,

    pub next: String,
    pub first_move: String,
    pub order: Vec<String>,

    pub players: Vec<Player>,

    pub moves: MovesData,
    pub available_moves: Vec<String>,
}

impl Game {
    pub fn get_player_mut(&mut self, player_name: &String) -> &mut Player {
        println!("{:?}", self.players);
        self.players
            .iter_mut()
            .find(|p| p.name == *player_name)
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
            .map(|m| get_from_string(&m).unwrap())
            .collect()
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