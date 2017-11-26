#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::fs;

mod constants;
mod models;

#[derive(Serialize, Deserialize, Debug)]
struct TestConfig {
    name: Option<models::config::MoveType>,
}


fn main() {
    let file = fs::File::open("src/config.json")
        .expect("file not found");

    let config: TestConfig = serde_json::from_reader(file).unwrap();
    println!("{:?}", config);


    let file = fs::File::create("src/other_config.json").unwrap();
    serde_json::to_writer(file, &config).unwrap();


    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        println!("{:?}", path.unwrap().file_name());
    }

    let x = constants::ROOM;
}
