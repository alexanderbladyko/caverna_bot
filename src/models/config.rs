#[derive(Serialize, Deserialize, Debug)]
pub enum MoveType {
    GnomeBirth,
    CavernBuild,
    RoomBuild,
    Basic,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MoveCondition {
    And,
    Or,
    AndThen,
}

pub struct MoveConfig {
    moveType: MoveType,
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Move {
    name: String,
    op1: MoveType,
    op2: Option<MoveType>,
    cond: Option<MoveCondition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {

}