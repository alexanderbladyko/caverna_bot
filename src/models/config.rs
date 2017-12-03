//#[derive(Serialize, Deserialize, Debug)]
//pub enum MoveType {
//    GnomeBirth,
//    CavernBuild,
//    RoomBuild,
//    Basic,
//}
//
//impl MoveType {
//    fn parse(&self, enum_str: &str) -> Result<MoveType, ()> {
//        return match enum_str {
//            "GnomeBirth" => Ok(MoveType::GnomeBirth),
//            _ => Err(()),
//        }
//    }
//}
//
//#[derive(Serialize, Deserialize, Debug)]
//pub enum MoveCondition {
//    And,
//    Or,
//    AndThen,
//}
//
//#[derive(Serialize, Deserialize, Debug)]
//pub struct Move {
//    name: String,
//    op1: MoveType,
//    op2: Option<MoveType>,
//    cond: Option<MoveCondition>,
//}
//
//#[derive(Serialize, Deserialize, Debug)]
//pub struct Config {
//
//}