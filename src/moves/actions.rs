use std::collections::HashMap;

use constants::GameStatus;
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
        format!("Updating resources {:?} for {:?}", self.update_hash, self.player)
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
        format!("Building rooms {:?} for {:?}", self.rooms, self.player)
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
        format!("Building fields {:?} for {:?}", self.fields, self.player)
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
        format!("Spawning new gnome for {:?}", self.player)
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
        format!("Next user is {:?}", self.player)
    }
}

pub struct FirstPlayer {
    pub player: String,
}

impl MoveAction for FirstPlayer {
    fn perform(&self, game: &mut Game) {
        let old_order = game.order.clone();

        let position = old_order
            .iter()
            .position(|p| *p == self.player)
            .unwrap();

        let before = old_order.into_iter().take(position).collect::<Vec<_>>();
        let mut after = game.order.clone().into_iter().skip(position).collect::<Vec<_>>();

        after.extend(before);

        game.order = after;
    }

    fn get_info(&self) -> String {
        format!("First player is {:?}", self.player)
    }
}

pub struct ReserveGnome {
    pub player: String,
}

impl MoveAction for ReserveGnome {
    fn perform(&self, game: &mut Game) {
        game.get_player_mut(&self.player).moved_gnomes += 1;
    }

    fn get_info(&self) -> String {
        format!("Reserve gnome for {:?}", self.player)
    }
}

pub struct BlockMove {
    pub player_move: String,
}

impl MoveAction for BlockMove {
    fn perform(&self, game: &mut Game) {
        let index = game.available_moves
            .iter()
            .position(|x| *x == self.player_move)
            .unwrap();
        game.available_moves.remove(index);
    }

    fn get_info(&self) -> String {
        format!("Blocking move {:?}", self.player_move)
    }
}

pub struct ChangeStatus {
    pub status: GameStatus,
}

impl MoveAction for ChangeStatus {
    fn perform(&self, game: &mut Game) {
        game.status = self.status.clone();
    }

    fn get_info(&self) -> String {
        format!("Changing game status {:?}", self.status)
    }
}