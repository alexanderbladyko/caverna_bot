use std::fs;
use std::path;
use serde_yaml;

use config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct DriftMining {
    pub coal_incr: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logging {
    pub wood_incr: u32,
    pub secondary_wood_incr: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WoodGathering {
    pub wood_incr: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Excavation {
    pub stone_incr: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MovesConfig {
    pub drift_mining: DriftMining,
    pub logging: Logging,
    pub wood_gathering: WoodGathering,
    pub excavation: Excavation,
}

impl MovesConfig {
    pub fn read_from_yaml(config: &Config, name: String) -> MovesConfig {
        let file = fs::File::open(path::Path::new(&config.folder).join(name))
            .expect("Error reading moves config file");
        serde_yaml::from_reader(file).unwrap()
    }
}

