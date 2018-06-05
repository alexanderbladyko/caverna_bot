#[cfg(test)]
mod test {
    #[cfg(test)]
    mod test_get_available_slots {
        use std::collections::HashSet;

        use utils::get_available_slots;

        #[test]
        fn test_free_slots() {
            let actual: HashSet<u32> = get_available_slots(vec![0, 1]);
            assert_eq!(actual, hash_set! {
                2, 3, 4
            });
        }

        #[test]
        fn test_long_free_slots() {
            let actual: HashSet<u32> = get_available_slots(vec![0, 1, 2, 4, 7]);
            assert_eq!(actual, hash_set! {
                3, 5, 6, 8, 10
            });
        }

        #[test]
        fn test_side_free_slots() {
            let actual: HashSet<u32> = get_available_slots(vec![0, 1, 2, 5, 8]);
            assert_eq!(actual, hash_set! {
                3, 4, 7, 11
            });
        }
    }

    #[cfg(test)]
    mod test_get_available_pair_slots {
        use std::collections::HashSet;

        use utils::get_available_pair_slots;

        #[test]
        fn test_free_pair_slots() {
            let actual = get_available_pair_slots(vec![0, 1]);
            assert_eq!(actual, hash_set! {
                (3, 6),
                (3, 4),
                (4, 7),
                (4, 5),
                (2, 5)
            });
        }

        #[test]
        fn test_free_pair_slots_complex() {
            let actual = get_available_pair_slots(vec![0, 1, 2, 5, 8]);
            assert_eq!(actual, hash_set! {
                (3, 4),
                (3, 6),
                (4, 7),
                (6, 7),
                (7, 10),
                (10, 11)
            });
        }
    }
}