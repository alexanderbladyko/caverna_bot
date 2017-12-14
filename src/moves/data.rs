#[derive(Serialize, Deserialize, Debug)]
pub struct MovesData {
    pub drift_mining: DriftMiningData,
    pub logging: LoggingData,
    pub wood_gathering: WoodGatheringData,
    pub excavation: ExcavationData,
    pub starting_player: StartingPlayerData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriftMiningData {
    pub stone: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingData {
    pub wood: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WoodGatheringData {
    pub wood: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExcavationData {
    pub stone: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartingPlayerData {
    pub food: u32,
}