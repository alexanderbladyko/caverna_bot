use std::fs;
use std::path;
use serde_yaml;

use config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct DriftMining {
    pub coal_incr: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MovesConfig {
    pub drift_mining: DriftMining,
}

impl MovesConfig {
    pub fn read_from_yaml(config: &Config, name: String) -> MovesConfig {
        let file = fs::File::open(path::Path::new(&config.folder).join(name))
            .expect("Error reading moves config file");
        serde_yaml::from_reader(file).unwrap()
    }
}

