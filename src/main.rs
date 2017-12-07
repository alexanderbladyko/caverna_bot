#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_yaml;
extern crate clap;

pub mod constants;
pub mod models;
pub mod moves;
pub mod test;

use std::fs;
use std::path;
use clap::{App, SubCommand, Arg};

use models::game::{Game};


#[derive(Serialize, Deserialize, Debug)]
struct Config {
    folder: String,
}


fn main() {
    let config: Config = _read_config_from_file();

    let (last_game_file, next_game_file) = _get_last_game_file(&config);

    let mut game: Game = _read_game_data_from_file(&config, last_game_file);

    game.turn += 1;

    let matches = App::new("Caverna bot")
        .version("1.0")
        .subcommand(SubCommand::with_name("show")
            .about("display game state"))
        .subcommand(SubCommand::with_name("decide")
            .about("make decision"))
        .get_matches();

    match matches.subcommand_name() {
        Some("show") => println!("{:?}", game),
        Some("write") => _write_game_data_to_file(&config, next_game_file, &game),
        Some(_) => return,
        None => return,
    };
}

fn _get_last_game_file(config: &Config) -> (String, String) {
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


fn _read_config_from_file() -> Config {
    let file = fs::File::open("config.yml")
        .expect("Config file not found");
    serde_yaml::from_reader(file).unwrap()
}

fn _read_game_data_from_file(config: &Config, path: String) -> Game {
    let file = fs::File::open(path::Path::new(&config.folder).join(path))
        .expect("Error reading game file");
    serde_yaml::from_reader(file).unwrap()
}

fn _write_game_data_to_file(config: &Config, path: String, game: &Game) {
    let file = fs::File::create(path::Path::new(&config.folder).join(path))
        .expect("Error game file");
    serde_yaml::to_writer(file, &game).unwrap()
}