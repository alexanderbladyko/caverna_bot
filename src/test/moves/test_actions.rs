#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use test::base;

    use constants;
    use moves::actions::{MoveAction, UpdateResources};


    #[test]
    fn test_changing_resources() {
        let mut game = base::get_game_with_2_players();
        let mut update_hash = HashMap::new();
        update_hash.insert(String::from(constants::ResourceType::Gold.str_key()), 30);

        let action = UpdateResources {
            player: String::from("p1"),
            update_hash,
        };
        action.perform(&mut game);

        base::assert_player_has_resource(&game, String::from("p1"), constants::ResourceType::Gold, 30);

    }
}