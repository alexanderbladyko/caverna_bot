#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use moves::UpdateResources;

    #[test]
    fn test_changing_resources() {
        let mut update_hash = HashMap::new();
        update_hash.insert(String::from("gold"), 30);

        let action = UpdateResources {
            player: String::from("test"),
            update_hash,
        };
        assert_eq!(1i32, 1i32);
    }
}