#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MovesData {
    pub drift_mining: DriftMiningData,
    pub logging: LoggingData,
    pub wood_gathering: WoodGatheringData,
    pub excavation: ExcavationData,
    pub clearing: ClearingData,
    pub starting_player: StartingPlayerData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DriftMiningData {
    pub stone: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggingData {
    pub wood: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WoodGatheringData {
    pub wood: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExcavationData {
    pub stone: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClearingData {
    pub wood: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StartingPlayerData {
    pub food: u32,
}