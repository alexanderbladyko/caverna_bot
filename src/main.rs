#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_yaml;
extern crate clap;

#[macro_use]
pub mod common;

pub mod config;
pub mod constants;
pub mod models;
pub mod moves;
pub mod rooms;
pub mod test;

use clap::{App, SubCommand, Arg};

use config::{Config};
use models::game::{Game};
use moves::config::{MovesConfig};
use moves::core::{Move, get_from_string, collect_actions};


fn main() {
    let config = Config::read_from_yaml();

    let (last_game_file, next_game_file) = Game::get_last_game_file(&config);

    let mut game: Game = Game::read_from_yaml(&config, last_game_file);

    let moves_config = MovesConfig::read_from_yaml(&config, String::from("moves_config.yml"));

    let mut app = App::new("Caverna bot")
        .version("1.0");
    app = app.subcommand(SubCommand::with_name("show")
        .about("display game state"));
    app = app.subcommand(SubCommand::with_name("decide")
        .about("make decision"));

    let available_moves = game.get_free_moves();
    let sub_commands: Vec<App<'static, 'static>> = available_moves
        .iter()
        .map(|m| m.get_sub_command())
        .collect();

    for cmd in sub_commands {
        app = app.subcommand(cmd);
    }
    let matches = app.get_matches();

    match matches.subcommand() {
        ("show", Some(m)) => {
            println!("{:?}", game);
            println!("{:?}", moves_config);

        },
        ("write", Some(m)) => game.write_to_yaml(&config, next_game_file),
        ("decide", Some(m)) => {

        },
        (name, Some(cmd)) => {
            match get_from_string(name) {
                Ok(mov) => {
                    mov.get_actions(game.clone(), &moves_config)
                        .iter()
                        .for_each(|a| println!("{:?}", a.get_info()));
                },
                Err(_) => panic!(format!("Not found implementation for command: {}", name)),
            }

        },
        _ => return,
    };
}
