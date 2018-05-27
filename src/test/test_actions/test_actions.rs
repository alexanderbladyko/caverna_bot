#[cfg(test)]
mod test {

    // Player move actions

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
    mod test_build_rooms {
        use test::base;

        use actions::{MoveAction, BuildRooms};
        use models::game::{PlayerRoom};
        use rooms::{constants as RoomConstants};

        #[test]
        fn test_perform() {
            let mut game = base::get_game_with_2_players();
            let rooms = vec![
                PlayerRoom {
                    room_type: String::from(RoomConstants::DWELLING),
                    position: 2,
                }
            ];

            let action = BuildRooms {
                player: String::from("p1"),
                rooms,
            };
            action.perform(&mut game);

            let player = game.get_player("p1");

            assert_eq!(player.rooms.len(), 1);
            assert_eq!(player.rooms[0].position, 2);
            assert_eq!(player.rooms[0].room_type, String::from(RoomConstants::DWELLING));
        }
    }

    #[cfg(test)]
    mod test_build_fields {
        use test::base;

        use actions::{MoveAction, BuildFields};
        use constants::{OutsideElement};
        use models::game::{PlayerField};

        #[test]
        fn test_perform() {
            let mut game = base::get_game_with_2_players();
            let fields = vec![
                PlayerField {
                    field_type: OutsideElement::Field,
                    position: 3,
                }
            ];

            let action = BuildFields {
                player: String::from("p1"),
                fields,
            };
            action.perform(&mut game);

            let player = game.get_player("p1");

            assert_eq!(player.fields.len(), 1);
            assert_eq!(player.fields[0].position, 3);
            assert_eq!(player.fields[0].field_type, OutsideElement::Field);
        }
    }

    #[cfg(test)]
    mod test_spawn_gnome {
        use test::base;

        use actions::{MoveAction, SpawnGnome};

        #[test]
        fn test_perform() {
            let mut game = base::get_game_with_2_players();

            let action = SpawnGnome {
                player: String::from("p1"),
            };
            action.perform(&mut game);

            let player = game.get_player("p1");
            assert_eq!(player.child_gnomes, 1);
        }
    }

    #[cfg(test)]
    mod test_set_first_player {
        use test::base;

        use actions::{MoveAction, SetFirstPlayer};

        #[test]
        fn test_perform() {
            let mut game = base::get_game_with_2_players();

            let action = SetFirstPlayer {
                player: String::from("p3"),
            };
            action.perform(&mut game);

            assert_eq!(game.first_move, String::from("p3"));
        }
    }

    // Game move actions

    #[cfg(test)]
    mod test_reorder_players {
        use test::base;

        use actions::{MoveAction, ReorderPlayers};

        #[test]
        fn test_perform() {
            let mut game = base::get_game_with_2_players();

            game.order = vec![
                String::from("p1"),
                String::from("p2"),
                String::from("p3"),
                String::from("p4"),
            ];

            let action = ReorderPlayers {
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
    mod test_increase_turn {
        use test::base;

        use actions::{MoveAction, IncreaseTurn};

        #[test]
        fn test_perform() {
            let mut game = base::get_game_with_2_players();

            game.turn = 10;

            let action = IncreaseTurn {};
            action.perform(&mut game);

            assert_eq!(game.turn, 11);
        }
    }

    #[cfg(test)]
    mod test_reserve_gnome {
        use test::base;

        use actions::{MoveAction, ReserveGnome};

        #[test]
        fn test_perform() {
            let mut game = base::get_game_with_2_players();

            let action = ReserveGnome {
                player: String::from("p1"),
            };
            action.perform(&mut game);

            let player = game.get_player("p1");
            assert_eq!(player.moved_gnomes, 1);
        }
    }

    #[cfg(test)]
    mod test_block_move {
        use test::base;

        use actions::{MoveAction, BlockMove};
        use moves::{constants as MovesConstants};

        #[test]
        fn test_perform() {
            let mut game = base::get_game_with_2_players();

            let action = BlockMove {
                player: String::from("p1"),
                player_move: String::from(MovesConstants::DONKEY_FARMING)
            };
            action.perform(&mut game);

            let player = game.get_player("p1");
            assert_eq!(player.moves, vec![String::from(MovesConstants::DONKEY_FARMING)]);
        }
    }

    #[cfg(test)]
    mod test_change_status {
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

    #[cfg(test)]
    mod test_next_user {
        use test::base;

        use actions::{MoveAction, NextUser};

        #[test]
        fn test_perform() {
            let mut game = base::get_game_with_2_players();

            game.next = String::from("p1");

            let action = NextUser {
                player: String::from("p2"),
            };
            action.perform(&mut game);

            assert_eq!(game.next, String::from("p2"));
        }
    }

    #[cfg(test)]
    mod test_release_moves {
        use test::base;

        use actions::{MoveAction, ReleaseMoves};
        use moves::{constants as MovesConstants};

        #[test]
        fn test_perform() {
            let mut game = base::get_game_with_2_players();

            {
                let p1 = game.get_player_mut(&String::from("p1"));
                p1.moves.push(String::from(MovesConstants::DONKEY_FARMING));
            }
            {
                let p2 = game.get_player_mut(&String::from("p2"));
                p2.moves.push(String::from(MovesConstants::ORE_DELIVERY));
            }

            let action = ReleaseMoves {};
            action.perform(&mut game);

            assert_eq!(game.get_player("p1").moves.len(), 0);
            assert_eq!(game.get_player("p2").moves.len(), 0);
        }
    }

    #[cfg(test)]
    mod test_open_new_move {
        use test::base;

        use actions::{MoveAction, OpenNewMove};
        use moves::{constants as MovesConstants};

        #[test]
        fn test_perform() {
            let mut game = base::get_game_with_2_players();

            let action = OpenNewMove {
                new_move: String::from(MovesConstants::BLACKSMITHING)
            };
            action.perform(&mut game);

            assert_eq!(*game.available_moves.last().unwrap(), String::from(MovesConstants::BLACKSMITHING));
        }
    }

    #[cfg(test)]
    mod test_set_feeding_and_breeding_status {
        use test::base;

        use actions::{MoveAction, SetFeedingAndBreedingStatus};
        use constants::{FeedingAndBreedingStatus};

        #[test]
        fn test_perform() {
            let mut game = base::get_game_with_2_players();

            let action = SetFeedingAndBreedingStatus {
                status: FeedingAndBreedingStatus::NoBreeding,
            };
            action.perform(&mut game);

            assert_eq!(game.feeding_and_breeding_status, FeedingAndBreedingStatus::NoBreeding);
        }
    }
}