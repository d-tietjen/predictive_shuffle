#[cfg(test)]

mod tests {
    use std::collections::{HashMap, HashSet};

    // #[test]
    // fn collision_test() {
    //     let size = 1000;
    //     let seed = b"1029493 357rewqwer6r5w".to_vec();

    //     let mut map = HashMap::new();
    //     for mut index in 0..size {
    //         index = predictive_shuffle::shuffle_prediction(index, &seed, size);
    //         assert!(!map.contains_key(&index));
    //         map.insert(index, ());
    //         // println!("{int}:{index}");
    //     }
    // }

    #[test]
    fn collision_skip_test() {
        let size = 1_000_000;
        let seed = b"1029493 357rewqwer6r5w".to_vec();
        let randomness = 1.0;

        let map = predictive_shuffle::skip_multi_index_shuffle_prediction(
            &(0..size).collect(),
            &seed,
            size,
            randomness,
        );
        assert_eq!(map.len(), size);
        let mut new_map = HashSet::new();
        for (_key, value) in map {
            assert!(!new_map.contains(&value));
            new_map.insert(value);
        }
        // println!("{map:#?}")
    }

    // #[test]
    // fn collision_test_vec() {
    //     let size = 1000;
    //     let seed = b"1029493 357rewqwer6r5w".to_vec();

    //     let mut map = HashMap::new();
    //     for mut index in 0..size {
    //         index = predictive_shuffle::shuffle_prediction(index, &seed, size);
    //         assert!(!map.contains_key(&index));
    //         map.insert(index, ());
    //         // println!("{int}:{index}");
    //     }
    // }

    // #[test]
    // fn multi_index() {
    //     let size = 5;
    //     let mut map = HashMap::new();
    //     for i in 0..5 {
    //         map.insert(i, i);
    //     }

    //     println!("{map:?}");
    //     let seed = b"seed".to_vec();
    //     let mut hash = predictive_shuffle::hash(&seed);
    //     hash.truncate(32);
    //     let seed = predictive_shuffle::demo(hash);

    //     // let mut rng = ChaCha8Rng::from_seed(seed);
    //     // let rand: usize = rng.gen_range(0..size);

    //     let mut new_map = HashMap::new();

    //     for i in (0..size).rev() {
    //         let x = if i == 0 { 0 } else { rand % i };

    //         if let Some((_index, origin)) = map.remove_entry(&x) {
    //             new_map.insert(origin, i);
    //             if map.is_empty() {
    //                 break;
    //             }
    //         }
    //         if let Some((_index, origin)) = map.remove_entry(&i) {
    //             map.insert(x, origin);
    //         }
    //         println!("{map:?}");
    //         // if x == index {
    //         //     index = i;
    //         //     break;
    //         // } else if i == index {
    //         //     index = x
    //         // };
    //     }

    //     println!("{new_map:?}");
    // }
}
