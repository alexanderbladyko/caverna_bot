#[cfg(test)]
mod test {
    #[cfg(test)]
    mod test_get_free_slots {
        use test::base;

        use moves::core::{Move, DriftMining};

        #[test]
        fn test_get_all_actions() {
            let player_move = DriftMining{};
            let game = base::get_game_with_2_players();
            let moves_config = base::get_moves_config();

            let actions = player_move.get_all_actions(game, &moves_config);


        }
    }
}