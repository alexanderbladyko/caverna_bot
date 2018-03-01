pub mod calculator {
    use constants::{ResourceType};
    use models::game::{Game};

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

    pub static ANIMALS: &'static [ResourceType] = &[
        ResourceType::Sheep,
        ResourceType::Hippo,
        ResourceType::Donkey,
        ResourceType::Cow,
    ];

    pub fn get_final_score(game: Game, player_name: &str) -> i32 {
        let mut score: i32 = 0;

        let player = game.get_player(&String::from(player_name));

        for animal in ANIMALS {
            if *player.resources.get(&animal.clone().str_key()).unwrap_or(&0) == 0 {
                score -= 2;
            } else {
                score += 1
            }
        }
        score += player.get_resource(ResourceType::Dog) as i32;
        score += player.get_resource(ResourceType::Pumpkin) as i32;
        score += (0.5 * player.get_resource(ResourceType::Wheat) as f32).ceil() as i32;
        score += player.get_resource(ResourceType::Gem) as i32;
        score += player.get_resource(ResourceType::Gold) as i32;
        score += player.gnomes as i32 + player.child_gnomes as i32;

        score += player.fines as i32 * -3;

        let rooms_score: u32 = player
            .get_rooms()
            .iter()
            .map(|r| r.get_score_points())
            .sum();

        score += rooms_score as i32;

        // TODO: add score from rooms, fields and caverns

        score
    }
}
