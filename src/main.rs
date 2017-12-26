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
pub mod score;
pub mod test;

use clap::{App, SubCommand, Arg, ArgMatches};

use config::{Config};
use models::game::{Game};
use moves::config::{MovesConfig};
use moves::core::{Move, get_from_string, collect_actions};


fn main() {
    let config = Config::read_from_yaml();

    let (last_game_file, next_game_file) = Game::get_last_game_file(&config);

    let game: &mut Game = &mut Game::read_from_yaml(&config, last_game_file);

    let moves_config = MovesConfig::read_from_yaml(&config, String::from("moves_config.yml"));

    let mut app = App::new("Caverna bot")
        .version("1.0");
    app = app.subcommand(SubCommand::with_name("show")
        .about("display game state"));
    app = app.subcommand(SubCommand::with_name("decide")
        .about("make decision"));
    app = app.subcommand(SubCommand::with_name("next_round")
        .about("calculates next turn")
        .arg(Arg::with_name("dry_run")
            .help("Dry run")
            .long("dry_run")
            .short("d")
        ).arg(Arg::with_name("new_move")
            .help("New move")
            .long("new_move")
            .short("n")));

    {
        let available_moves = game.get_free_moves();
        let sub_commands: Vec<App<'static, 'static>> = available_moves
            .iter()
            .map(|m| m.get_sub_command())
            .collect();

        for cmd in sub_commands {
            app = app.subcommand(cmd.arg(Arg::with_name("dry_run")
                .help("Dry run")
                .long("dry_run")
                .short("d")
            ));
        }
    }
    let matches = app.get_matches();

    match matches.subcommand() {
        ("show", Some(m)) => {
            println!("{:?}", game);
            println!("{:?}", moves_config);
        },
        ("decide", Some(m)) => {
            _decide(game, &moves_config);
        },
        ("next_round", Some(cmd)) => {
            _next_round_game(cmd, game, &config, &moves_config, next_game_file);
        },
        (name, Some(cmd)) => {
            _perform_move(&name, cmd, game, &config, &moves_config, next_game_file);
        },
        _ => return,
    };
}

fn _decide(game: &mut Game, moves_config: &MovesConfig) {
    if game.status != constants::GameStatus::PlayerMove {
        panic!("Status is not '{:?}'", constants::GameStatus::PlayerMove);
    }
    game.available_moves
        .iter()
        .for_each(|m| {
            match get_from_string(m) {
                Ok(mov) => {
                    println!("Exploring '{:?}'", m);
                    let actions = mov.get_all_actions(game.clone(), &moves_config);
                    actions
                        .iter()
                        .for_each(|a| println!("{:?} - {:?}", a.weight, a.get_info()));
                    let info = actions
                        .iter()
                        .max_by_key(|a| a.weight)
                        .unwrap()
                        .get_info();
                    println!("Max {:?}", info);
                },
                Err(_) => panic!(format!("Not found implementation for command: {}", m)),
            }
        });
}

fn _perform_move(name: &str, cmd: &ArgMatches, game: &mut Game, config: &Config,
                 moves_config: &MovesConfig, output_file: String
) {
    if game.status != constants::GameStatus::PlayerMove {
        panic!("Status is not '{:?}'", constants::GameStatus::PlayerMove);
    }
    match get_from_string(name) {
        Ok(mov) => {
            let actions = mov.get_actions(game.clone(), &moves_config, &cmd);
            println!("{:?}", actions.get_info());

            if cmd.occurrences_of("dry_run") == 0 {
                println!("Applying changes");
                actions.perform(game);
                game.write_to_yaml(&config, output_file);
            } else {
                println!("Dry run");
            }
        },
        Err(_) => panic!(format!("Not found implementation for command: {}", name)),
    }
}

fn _next_round_game(cmd: &ArgMatches, game: &mut Game, config: &Config,
                    moves_config: &MovesConfig, output_file: String) {
    if game.status != constants::GameStatus::NextTurnPending {
        panic!("Status is not '{:?}'", constants::GameStatus::NextTurnPending);
    }

    game.status = constants::GameStatus::PlayerMove;
    game.next = game.order.first().unwrap().to_string();

    match cmd.value_of("new_move") {
        Some(new_move) => match get_from_string(new_move) {
            Ok(mov) => {
                for mov in game.clone().get_all_moves() {
                    mov.on_next_turn(game, &moves_config);
                }

                if cmd.occurrences_of("dry_run") == 0 {
                    println!("Applying changes");
                    game.write_to_yaml(&config, output_file);
                } else {
                    println!("Dry run");
                }
            }
            Err(_) => panic!(format!("Not found implementation for command: {}", new_move)),
        },
        _ => {},
    }
}
