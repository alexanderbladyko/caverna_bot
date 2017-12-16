#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_yaml;
extern crate clap;

pub mod config;
pub mod constants;
pub mod models;
pub mod moves;
pub mod test;

use clap::{App, SubCommand, Arg};

use config::{Config};
use models::game::{Game};
use moves::config::{MovesConfig};
use moves::core::{Moves, Move, get_from_string};


fn main() {
    let config = Config::read_from_yaml();

    let (last_game_file, next_game_file) = Game::get_last_game_file(&config);

    let mut game: Game = Game::read_from_yaml(&config, last_game_file);

    let moves_config = MovesConfig::read_from_yaml(&config, String::from("config.yml"));

    let mut app = App::new("Caverna bot")
        .version("1.0");
    app = app.subcommand(SubCommand::with_name("show")
        .about("display game state"));
    app = app.subcommand(SubCommand::with_name("decide")
        .about("make decision"));

    for game_move in game.get_free_moves() {
        app = app.subcommand(game_move.get_sub_command());
    }

    let matches = app.get_matches();

    let moves = Moves {
        game,
        moves_config,
        actions: Vec::new(),
    };

    match matches.subcommand_name() {
        Some("show") => {
            println!("{:?}", moves.game);
            println!("{:?}", moves.moves_config);

        },
        Some("write") => moves.game.write_to_yaml(&config, next_game_file),
        Some("decide") => {

        },
        Some(_) => return,
        None => return,
    };
}
