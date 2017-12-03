pub enum InsideElement {
    Room,
    Hall,
    MineHall,
    Mine,
    GemMine,
}


pub enum OutsideElement {
    Meadow,
    Field,
    MeadowField,
    FenceField,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResourceType {
    Gem,
    Food,
    Gold,
    Stone,
    Wood,
    Coal,
    Sheep,
    Hippo,
    Dog,
    Donkey,
    Cow,
    Wheat,
    Pumpkin,
}