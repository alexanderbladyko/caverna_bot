pub mod constants;

use std::any::Any;
use std::collections::HashMap;

use actions::{constants as ActionsConstants};
use constants::GameStatus;
use models::game::{Game, PlayerRoom, PlayerField};

pub trait MoveAction {
    fn get_name(&self) -> &str;

    fn perform(&self, game: &mut Game);

    fn get_info(&self) -> String;

    fn as_any(&self) -> &Any;
}


// ----- Player actions -----
pub struct Actions {
    pub actions: Vec<Box<MoveAction>>,
    pub args: HashMap<String, String>,
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

    pub fn from_vec(actions: Vec<Box<MoveAction>>) -> Actions {
        Actions {
            args: HashMap::new(),
            actions,
        }
    }
}

#[derive(Clone)]
pub struct UpdateResources {
    pub player: String,
    pub update_hash: HashMap<String, u32>,
}

impl MoveAction for UpdateResources {
    fn get_name(&self) -> &str {
        ActionsConstants::UPDATE_RESOURCES
    }

    fn perform(&self, game: &mut Game) {
        game
            .get_player_mut(&self.player)
            .change_resources(self.update_hash.clone());
    }

    fn get_info(&self) -> String {
        format!("Updating resources {:?} for {:?}", self.update_hash, self.player)
    }

    fn as_any(&self) -> &Any {
        self
    }
}

#[derive(Clone)]
pub struct BuildRooms {
    pub player: String,
    pub rooms: Vec<PlayerRoom>,
}

impl MoveAction for BuildRooms {
    fn get_name(&self) -> &str {
        ActionsConstants::BUILD_ROOMS
    }

    fn perform(&self, game: &mut Game) {
        game
            .get_player_mut(&self.player)
            .add_rooms(self.rooms.clone());
    }

    fn get_info(&self) -> String {
        format!("Building rooms {:?} for {:?}", self.rooms, self.player)
    }

    fn as_any(&self) -> &Any {
        self
    }
}

#[derive(Clone)]
pub struct BuildFields {
    pub player: String,
    pub fields: Vec<PlayerField>,
}

impl MoveAction for BuildFields {
    fn get_name(&self) -> &str {
        ActionsConstants::BUILD_FIELDS
    }

    fn perform(&self, game: &mut Game) {
        game.get_player_mut(&self.player).add_fields(self.fields.clone());
    }

    fn get_info(&self) -> String {
        format!("Building fields {:?} for {:?}", self.fields, self.player)
    }

    fn as_any(&self) -> &Any {
        self
    }
}

#[derive(Clone)]
pub struct SpawnGnome {
    pub player: String,
}

impl MoveAction for SpawnGnome {
    fn get_name(&self) -> &str {
        ActionsConstants::SPAWN_GNOME
    }

    fn perform(&self, game: &mut Game) {
        game.get_player_mut(&self.player).spawn_new_gnome();
    }

    fn get_info(&self) -> String {
        format!("Spawning new gnome for {:?}", self.player)
    }

    fn as_any(&self) -> &Any {
        self
    }
}

#[derive(Clone)]
pub struct SetFirstPlayer {
    pub player: String,
}

impl MoveAction for SetFirstPlayer {
    fn get_name(&self) -> &str {
        ActionsConstants::SET_FIRST_PLAYER
    }

    fn perform(&self, game: &mut Game) {
        game.first_move = self.player.clone();
    }

    fn get_info(&self) -> String {
        format!("Next first player is {:?}", self.player)
    }

    fn as_any(&self) -> &Any {
        self
    }
}

// ----- Game actions -----

#[derive(Clone)]
pub struct FirstPlayer {
    pub player: String,
}

impl MoveAction for FirstPlayer {
    fn get_name(&self) -> &str {
        ActionsConstants::FIRST_PLAYER
    }

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

    fn as_any(&self) -> &Any {
        self
    }
}

#[derive(Clone)]
pub struct IncreaseTurn {}

impl MoveAction for IncreaseTurn {
    fn get_name(&self) -> &str {
        ""
    }

    fn perform(&self, game: &mut Game) {
        game.turn += 1;
    }

    fn get_info(&self) -> String {
        format!("Game turn +1")
    }

    fn as_any(&self) -> &Any {
        self
    }
}

#[derive(Clone)]
pub struct ReserveGnome {
    pub player: String,
}

impl MoveAction for ReserveGnome {
    fn get_name(&self) -> &str {
        ""
    }

    fn perform(&self, game: &mut Game) {
        game.get_player_mut(&self.player).moved_gnomes += 1;
    }

    fn get_info(&self) -> String {
        format!("Reserve gnome for {:?}", self.player)
    }

    fn as_any(&self) -> &Any {
        self
    }
}

#[derive(Clone)]
pub struct BlockMove {
    pub player: String,
    pub player_move: String,
}

impl MoveAction for BlockMove {
    fn get_name(&self) -> &str {
        ""
    }

    fn perform(&self, game: &mut Game) {
        game.get_player_mut(&self.player).moves.push(self.player_move.clone());
    }

    fn get_info(&self) -> String {
        format!("Blocking move {:?}", self.player_move)
    }

    fn as_any(&self) -> &Any {
        self
    }
}

#[derive(Clone)]
pub struct ChangeStatus {
    pub status: GameStatus,
}

impl MoveAction for ChangeStatus {
    fn get_name(&self) -> &str {
        ""
    }

    fn perform(&self, game: &mut Game) {
        game.status = self.status.clone();
    }

    fn get_info(&self) -> String {
        format!("Changing game status {:?}", self.status)
    }

    fn as_any(&self) -> &Any {
        self
    }
}

#[derive(Clone)]
pub struct NextUser {
    pub player: String,
}

impl MoveAction for NextUser {
    fn get_name(&self) -> &str {
        ""
    }

    fn perform(&self, game: &mut Game) {
        game.next = self.player.clone();
    }

    fn get_info(&self) -> String {
        format!("Next user is {:?}", self.player)
    }

    fn as_any(&self) -> &Any {
        self
    }
}


#[derive(Clone)]
pub struct ReleaseMoves {}

impl MoveAction for ReleaseMoves {
    fn get_name(&self) -> &str {
        ""
    }

    fn perform(&self, game: &mut Game) {
        game.players.iter_mut().for_each(|ref mut p| { p.moves.clear() });
    }

    fn get_info(&self) -> String {
        format!("Releasing all moves")
    }

    fn as_any(&self) -> &Any {
        self
    }
}