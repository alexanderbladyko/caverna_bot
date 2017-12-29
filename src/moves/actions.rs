use std::collections::HashMap;

use models::game::{Game, PlayerCavern, PlayerField};

pub trait MoveAction {
    fn perform(&self, game: &mut Game);

    fn get_info(&self) -> String;
}

pub struct Actions {
    pub weight: i64,
    pub actions: Vec<Box<MoveAction>>,
}

impl Actions {
    pub fn get_info(&self) -> Vec<String> {
        self.actions
            .iter()
            .map(|a| a.get_info())
            .collect()
    }

    pub fn perform(&self, game: &mut Game) {
        self.actions
            .iter()
            .for_each(|a| a.perform(game))
    }
}

pub struct UpdateResources {
    pub player: String,
    pub update_hash: HashMap<String, u32>,
}

impl MoveAction for UpdateResources {
    fn perform(&self, game: &mut Game) {
        game
            .get_player_mut(&self.player)
            .change_resources(self.update_hash.clone());
    }

    fn get_info(&self) -> String {
        format!("Updating resources {:?}", self.update_hash)
    }
}

pub struct BuildRooms {
    pub player: String,
    pub rooms: Vec<PlayerCavern>,
}

impl MoveAction for BuildRooms {
    fn perform(&self, game: &mut Game) {
        game
            .get_player_mut(&self.player)
            .add_rooms(self.rooms.clone());
    }

    fn get_info(&self) -> String {
        format!("Building rooms {:?}", self.rooms)
    }
}

pub struct BuildFields {
    pub player: String,
    pub fields: Vec<PlayerField>,
}

impl MoveAction for BuildFields {
    fn perform(&self, game: &mut Game) {
        game.get_player_mut(&self.player).add_fields(self.fields.clone());
    }

    fn get_info(&self) -> String {
        format!("Building fields {:?}", self.fields)
    }
}

pub struct SpawnGnome {
    pub player: String,
}

impl MoveAction for SpawnGnome {
    fn perform(&self, game: &mut Game) {
        game.get_player_mut(&self.player).spawn_new_gnome();
    }

    fn get_info(&self) -> String {
        format!("Spawning gnome")
    }
}

pub struct IncreaseTurn {}

impl MoveAction for IncreaseTurn {
    fn perform(&self, game: &mut Game) {
        game.turn += 1;
    }

    fn get_info(&self) -> String {
        format!("Game turn +1")
    }
}

pub struct NextUser {
    pub player: String,
}

impl MoveAction for NextUser {
    fn perform(&self, game: &mut Game) {
        game.next = self.player.clone();
    }

    fn get_info(&self) -> String {
        format!("Next user {:?}", self.player)
    }
}

pub struct FirstPlayer {
    pub player: String,
}

impl MoveAction for FirstPlayer {
    fn perform(&self, game: &mut Game) {
        let old_order = game.order.clone();

        let position = old_order.iter().position(|p| *p == self.player).unwrap();

        let before = old_order.iter().take(position).map(|p| *p).collect::<Vec<String>>();
        let mut after = old_order.into_iter().skip(position).collect::<Vec<_>>();

        after.extend(before);

        game.order = after;
    }

    fn get_info(&self) -> String {
        format!("First player is {:?}", self.player)
    }
}
