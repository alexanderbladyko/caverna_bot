pub mod calculator {
    use constants::{ResourceType};

    pub fn get_score(resource: ResourceType, count: u32) -> u32 {
        match resource {
            ResourceType::Dog => count,
            ResourceType::Wheat => (0.5 * count as f32).ceil() as u32,
            ResourceType::Pumpkin => count,
            ResourceType::Gold => count,
            ResourceType::Sheep => count,
            ResourceType::Hippo => count,
            ResourceType::Donkey => count,
            ResourceType::Cow => count,
            ResourceType::Gem => count,
            _ => 0,
        }
    }
}
