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

use std::fs;
use std::path;
use clap::{App, SubCommand, Arg};

use config::{Config};
use models::game::{Game};
use moves::config::{MovesConfig};


fn main() {
    let config = Config::read_from_yaml();

    let (last_game_file, next_game_file) = Game::get_last_game_file(&config);

    let mut game: Game = Game::read_from_yaml(&config, last_game_file);

    let moves_config = MovesConfig::read_from_yaml(&config, String::from("config.yml"));

    let matches = App::new("Caverna bot")
        .version("1.0")
        .subcommand(SubCommand::with_name("show")
            .about("display game state"))
        .subcommand(SubCommand::with_name("decide")
            .about("make decision"))
        .get_matches();

    match matches.subcommand_name() {
        Some("show") => {
            println!("{:?}", game);
            println!("{:?}", moves_config);

        },
        Some("write") => game.write_to_yaml(&config, next_game_file),
        Some(_) => return,
        None => return,
    };
}
