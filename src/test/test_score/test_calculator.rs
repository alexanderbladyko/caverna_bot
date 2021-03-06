#[cfg(test)]
mod test {

    #[cfg(test)]
    mod test_calculate_score {
        use test::base;

        use score::calculator;

        #[test]
        fn test_no_resources() {
            let game = base::get_game_with_2_players();
            let score = calculator::get_final_score(game.clone(), game.players.first().unwrap().name.as_str());
            assert_eq!(score, -6); // 2 gnomes, no animals and no resources
        }

        #[test]
        fn test_gnomes_effect_on_score() {
            let mut game = base::get_game_with_2_players().clone();
            game.get_player_mut(&String::from("p1")).gnomes += 1;

            let score = calculator::get_final_score(game.clone(), &game.next.as_str());
            assert_eq!(score, -5);
        }

        #[test]
        fn test_animals_effect_on_score() {
            let mut game = base::get_game_with_2_players().clone();
            game.get_player_mut(&String::from("p1")).gnomes += 1;

            let score = calculator::get_final_score(game.clone(), &game.next.as_str());
            assert_eq!(score, -5);
        }
    }
}