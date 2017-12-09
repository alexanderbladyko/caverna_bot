use std::fs;

use serde_yaml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub folder: String,
}


impl Config {
    pub fn read_from_yaml() -> Config {
        let file = fs::File::open("config.yml").expect("Config file not found");
        serde_yaml::from_reader(file).unwrap()
    }
}
