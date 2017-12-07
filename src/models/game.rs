use std::collections::{HashMap, HashSet};

use constants::{InsideElement, OutsideElement};


#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerRoom {
    pub room_type: InsideElement,
    pub position: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerField {
    pub field_type: OutsideElement,
    pub position: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,

    pub gnomes: u16,
    pub child_gnomes: u16,
    pub moved_gnomes: u16,

    pub rooms: Vec<PlayerRoom>,
    pub room_slots_count: u16,

    pub fields: Vec<PlayerField>,
    pub field_slots_count: u16,

    pub resources: HashMap<String, u32>,
}

impl Player {
    pub fn change_resources(&mut self, delta: HashMap<String, u32>) {
        for key in delta.keys().into_iter() {
            *self.resources.get_mut(key).unwrap() += *delta.get(key).unwrap();
        }
    }

    pub fn add_room_slots(&mut self, amount: u16) {
        self.room_slots_count += amount;
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
        let mut slots: HashSet<u16> = HashSet::from(
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
        let mut slots: HashSet<u16> = HashSet::from(
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub turn: u64,

    pub next: String,
    pub first_move: String,
    pub order: Vec<String>,

    pub players: Vec<Player>,
}
