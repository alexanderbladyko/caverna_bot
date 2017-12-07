#[derive(Serialize, Deserialize, Debug)]
pub enum InsideElement {
    Room,
    Hall,
    MineHall,
    Mine,
    GemMine,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OutsideElement {
    Meadow,
    Field,
    Fence,
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