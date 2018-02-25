#[cfg(test)]
mod test {

    #[cfg(test)]
    mod test_update_resources {
        use std::collections::HashMap;

        use test::base;

        use constants;
        use actions::{MoveAction, UpdateResources};

        #[test]
        fn test_perform() {
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

    #[cfg(test)]
    mod test_spawn_gnome {
        use std::collections::HashMap;

        use test::base;

        use constants;
        use actions::{MoveAction, SpawnGnome};

        #[test]
        fn test_perform() {
            let mut game = base::get_game_with_2_players();

            let action = SpawnGnome {
                player: String::from("p1"),
            };
            action.perform(&mut game);

            let player = game.players.iter().find(|p| p.name == String::from("p1")).unwrap();
            assert_eq!(player.child_gnomes, 1);
        }
    }

    #[cfg(test)]
    mod test_first_player {
        use std::collections::HashMap;

        use test::base;

        use constants;
        use actions::{MoveAction, FirstPlayer};

        #[test]
        fn test_perform() {
            let mut game = base::get_game_with_2_players();

            game.order = vec![
                String::from("p1"),
                String::from("p2"),
                String::from("p3"),
                String::from("p4"),
            ];

            let action = FirstPlayer {
                player: String::from("p3"),
            };
            action.perform(&mut game);

            assert_eq!(game.order, vec![
                String::from("p3"),
                String::from("p4"),
                String::from("p1"),
                String::from("p2"),
            ]);
        }
    }

    #[cfg(test)]
    mod test_reserve_gnome {
        use std::collections::HashMap;

        use test::base;

        use constants;
        use actions::{MoveAction, ReserveGnome};

        #[test]
        fn test_perform() {
            let mut game = base::get_game_with_2_players();

            let action = ReserveGnome {
                player: String::from("p1"),
            };
            action.perform(&mut game);

            let player = game.players.iter().find(|p| p.name == String::from("p1")).unwrap();
            assert_eq!(player.moved_gnomes, 1);
        }
    }

    #[cfg(test)]
    mod test_change_status {
        use std::collections::HashMap;

        use test::base;

        use constants;
        use actions::{MoveAction, ChangeStatus};

        #[test]
        fn test_perform() {
            let mut game = base::get_game_with_2_players();

            let action = ChangeStatus {
                status: constants::GameStatus::NextTurnPending,
            };
            action.perform(&mut game);

            assert_eq!(game.status, constants::GameStatus::NextTurnPending);
        }
    }
}