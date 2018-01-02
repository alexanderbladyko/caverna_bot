#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq)]
pub enum GameStatus {
    PlayerMove,
    NextTurnPending,
    GnomeFeeding,
    TribalBreeding,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InsideElement {
    Room,
    Hall,
    MineHall,
    Mine,
    GemMine,
}

impl InsideElement {
    pub fn str_key(self) -> String {
        String::from(match self {
            InsideElement::Room => "room",
            InsideElement::Hall => "hall",
            InsideElement::MineHall => "mine_hall",
            InsideElement::Mine => "mine",
            InsideElement::GemMine => "gem_mine",
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OutsideElement {
    Meadow,
    Field,
    Fence,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
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

pub static TRIBAL_ANIMALS: &'static [ResourceType] = &[
    ResourceType::Sheep,
    ResourceType::Hippo,
    ResourceType::Donkey,
    ResourceType::Cow,
];

impl ResourceType {
    pub fn str_key(self) -> String {
        String::from(match self {
            ResourceType::Gem => "gem",
            ResourceType::Food => "food",
            ResourceType::Gold => "gold",
            ResourceType::Stone => "stone",
            ResourceType::Wood => "wood",
            ResourceType::Coal => "coal",
            ResourceType::Sheep => "sheep",
            ResourceType::Hippo => "hippo",
            ResourceType::Dog => "dog",
            ResourceType::Donkey => "donkey",
            ResourceType::Cow => "cow",
            ResourceType::Wheat => "wheat",
            ResourceType::Pumpkin => "pumpkin",
        })
    }
}

