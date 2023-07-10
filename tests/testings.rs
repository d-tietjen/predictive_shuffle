#[cfg(test)]

mod tests {
    use std::collections::HashMap;
    include!(concat!(env!("OUT_DIR"), "/byte_lookup.rs"));

    #[test]
    fn collision_test() {
        let size = 100;
        let seed = b"hello world".to_vec();
        let mut map = HashMap::new();
        for index in 0..size {
            println!("index: {index}");

            let new_index = predictive_shuffle::shuffle(size, &seed, index);
            println!("new_index: {new_index}");
            assert!(!map.contains_key(&new_index));
            map.insert(new_index, index);
        }
    }

    #[test]
    fn shuffle() {
        let size = 100;
        let seed = b"hello world".to_vec();
        for index in 0..10 {
            let new_index = predictive_shuffle::shuffle(size, &seed, 2);
            println!("{new_index}");
        }
    }

    #[test]
    fn parsing() {
        // let mut bits = vec![];
        for byte in &BYTES {
            println!("{}", byte.1.trim_start().trim());
        }
        // for i in 0..8 {
        //     let mask = 1 << i;
        //     let bit = (mask & byte) > 0;
        //     bits.push(bit)
        // }
    }

    // #[test]
    // fn seed_to_bvec() {
    //     let mut hasher = Sha256::new();
    //     hasher.update(b"hash lll");
    //     let result = hasher.finalize();
    //     let _val = format!("{:x}", result).as_bytes().to_vec();
    // }

    #[test]
    fn prime_div() {
        // the function is able to divide any number by base 2 some number of times, allowing for the shuffling of irregular or prime numbers
        let len = 100;
        // let mut index = 4;
        let log = (len as f32).log2() as usize;
        // println!("log: {log}, index: {index}");

        for i in 0..len {
            let mut vec = vec![false, false, false, true, false, true, false, true];
            let mut index = i;
            let mut left;
            let mut right;
            while !vec.is_empty() {
                let mut start = 0;
                let mut size = len;
                for _ in 1..log {
                    if let Some(bool) = vec.pop() {
                        left = size / 2;
                        right = size - left;

                        match bool {
                            true => {
                                if index - start < left {
                                    index += right;
                                    size = right;
                                    start += left;
                                } else {
                                    index -= left;
                                    size = left;
                                }
                            }
                            false => {
                                if index - start < left {
                                    size = left;
                                } else {
                                    size = right;
                                    start += left;
                                }
                            }
                        }

                        // println!("Index: {index}, Start: {start} {left}, {right}");
                    } else {
                        break;
                    }
                }
            }
            // let new_index = predictive_shuffle::shuffle(i, &vec![false, false, false, true, false, true, false, true], index);
            print!("{index},")
        }
    }

    // #[test]
    // fn build() {
    //     const RADIX: u32 = 2;

    //     let mut map: HashMap<u8, Vec<bool>> = HashMap::new();
    //     for i in 0..=1 {
    //         let vec: Vec<bool> = format!("{:08b}", i)
    //             .chars()
    //             .map(|c| matches!(c.to_digit(RADIX).unwrap(), 1))
    //             .collect();
    //         map.insert(i, vec);
    //     }

    //     println!("{map:#?}")
    // }
}
