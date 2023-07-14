#[cfg(test)]

mod tests {
    use predictive_shuffle::Shuffle;
    use std::collections::HashSet;

    #[test]
    fn predictive_test() {
        let mut vec: Vec<usize> = (0..100).collect();
        let seed = b"seed phrase".to_vec();
        let positions: Vec<usize> = (0..100).collect();
        let shuffled_items = vec.predictive_shuffle_from_seed(positions, seed);

        let mut shuffled = HashSet::new();
        for (_, value) in shuffled_items {
            assert!(!shuffled.contains(&value));
            shuffled.insert(value);
        }
    }

    #[test]
    fn crypto_predictive_test() {
        let mut vec: Vec<usize> = (0..100).collect();
        let seed = b"seed phrase".to_vec();
        let positions: Vec<usize> = vec![1, 5];
        let shuffled_items = vec.crypto_predictive_shuffle_from_seed(positions, seed);
        println!("{:?}", shuffled_items);
    }

    #[test]
    fn batch_predictive_test() {
        let mut vec: Vec<usize> = (0..100).collect();
        let seed = b"seed phrase".to_vec();
        let positions: Vec<usize> = vec![1, 5];
        let batch = 4;
        let shuffled_items = vec.batch_predictive_shuffle_from_seed(batch, positions, seed);
        println!("{:?}", shuffled_items);
    }

    #[test]
    fn crypto_batch_predictive_test() {
        let mut vec: Vec<usize> = (0..100).collect();
        let seed = b"seed phrase".to_vec();
        let positions: Vec<usize> = vec![1, 5];
        let batch = 4;
        let shuffled_items = vec.crypto_batch_predictive_shuffle_from_seed(batch, positions, seed);
        println!("{:?}", shuffled_items);
    }
}
