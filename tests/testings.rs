#[cfg(test)]

mod tests {
    use std::collections::HashMap;
    include!(concat!(env!("OUT_DIR"), "/byte_lookup.rs"));

    #[test]
    fn collision_test() {
        let size = 50;
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
        let len = 10;
        let log = (len as f32).log2() as usize;

        let vec = vec![
            false, false, false, true, false, true, false, true, false, false, true, false, true,
            false, true, false, false, true, false, true, false, true, false, false, true, false,
            true, false, true, false, false, true,
        ];

        let mut results = vec![];
        for i in 0..len {
            let mut vec = vec.clone();
            let mut index = i;
            while !vec.is_empty() {
                let mut base = 0;
                let mut size = len;
                for _ in 1..log {
                    if let Some(bool) = vec.pop() {
                        size = size / 2;
                        if (size % 2) == 0 {
                            match bool {
                                true => {
                                    if index - base < size {
                                        base += size;
                                        index += size;
                                    } else {
                                        index -= size;
                                    }
                                }
                                false => {
                                    // do not move
                                    if index >= size {
                                        base += size;
                                    }
                                }
                            }

                            // if even
                        } else {
                            if index == size {
                                // break;
                                break;
                            }
                            match bool {
                                true => {
                                    if index - base < size {
                                        base += size + 1;
                                        index += size + 1;
                                    } else {
                                        index -= size + 1;
                                    }
                                }
                                false => {
                                    // do not move
                                    if index > size {
                                        base += size + 1;
                                    }
                                }
                            }
                        }
                        println!("{i} {index}");
                    } else {
                        continue;
                    }
                }
            }
            // println!("{index}");
            results.push(index);
        }

        let mut map: HashMap<&i32, ()> = HashMap::new();
        let mut bool = false;
        for i in &results {
            if map.contains_key(&i) {
                bool = true;
                break;
            } else {
                map.insert(i, ());
            }
        }
        println!("duplicates: {bool} {results:?}",);
    }

    #[test]
    fn new_shuffle() {
        let len = 10;
        let log = (len as f32).log2() as usize;
        let vector = vec![true];
        for id in 0..len {
            let mut bool = vector.clone();
            let mut index = id;
            // print!("{index},");
            while !bool.is_empty() {
                let mut size = len;
                let mut base = 0;
                for _ in 0..log {
                    if let Some(bool) = bool.pop() {
                        if (size % 2) == 0 {
                            size /= 2;
                            if index - base < size {
                                match bool {
                                    true => {
                                        base += size;
                                        index += size;
                                    }
                                    false => {}
                                }
                            } else {
                                match bool {
                                    true => {
                                        index -= size;
                                    }
                                    false => base += size,
                                }
                            }
                        } else {
                            size /= 2;
                            if index - base == size + 1 {
                                break;
                            } else if index - base < size {
                                match bool {
                                    true => {
                                        base += size + 1;
                                        index += size + 1;
                                    }
                                    false => {}
                                }
                            } else if index > size {
                                match bool {
                                    true => {
                                        base -= size - 1;
                                        index -= size - 1;
                                    }
                                    false => base += size + 1,
                                }
                            }
                        }
                    } else {
                        continue;
                    }
                }
            }
            print!("{index},");
        }
    }

    use std::cmp::Ordering::{Equal, Greater, Less};

    use predictive_shuffle::{index_selection, parse_binary};

    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    fn rand(seed: Vec<u8>) -> u32 {
        let seed = demo(seed);
        let mut rng = ChaCha8Rng::from_seed(seed);
        rng.gen()
    }

    use std::convert::TryInto;

    fn demo(v: Vec<u8>) -> [u8; 32] {
        v.try_into().unwrap_or_else(|v: Vec<u8>| {
            panic!("Expected a Vec of length {} but it was {}", 32, v.len())
        })
    }

    #[test]
    fn shuffle_prediction() {
        let size = 10000;
        let mut seed = b"1029493 string".to_vec();
        seed = predictive_shuffle::hash(seed);
        seed.truncate(32);
        let seed = demo(seed);

        let mut map = HashMap::new();
        for int in 0..size {
            let mut index = int;

            let mut rng = ChaCha8Rng::from_seed(seed);
            for i in (0..size).rev() {
                let x = rng.gen_range(0..=i);

                if x == index {
                    index = i;
                    break;
                } else if i == index {
                    index = x
                };
            }
            assert!(!map.contains_key(&index));
            map.insert(index, int);
            // println!("{int}:{index}");
        }
    }

    #[test]
    fn split() {
        // functioning impl
        let len = 10;
        let log = (len as f32).log2() as usize;
        println!("{log}");
        let mut bits = vec![];
        let mut seed = b"1029493 string".to_vec();
        seed = predictive_shuffle::hash(seed);
        // seed.truncate(32);
        for byte in &seed {
            if let Some(binary) = BYTES.get(byte) {
                bits.append(&mut parse_binary(binary))
            }
        }

        bits = vec![true, true];
        println!("{}", bits.len());

        // let rand: usize = rand(seed) as usize;

        let base = 0;
        let mut positions = vec![];
        for mut index in 0..len {
            // println!("INDEX: {index}");
            let mut bools = bits.clone();
            while !bools.is_empty() {
                let mut b = base;
                let mut d = 0;
                let mut size = len;
                'iter: for _ in 0..log {
                    if let Some(bool) = bools.pop() {
                        let output = index_selection(index, size, bool, b);
                        index = output.0;
                        if output.1 == Equal {
                            break 'iter;
                        }

                        let s = size / 2;
                        if (index - b) >= s + (size % 2) {
                            b += s + (size % 2)
                        }

                        // index += 1;
                        // index %= len;
                        // if index == 0 {
                        //     b = 0;
                        // } else if index >= b + s {
                        //     b += s
                        // }
                        // index %= len;

                        // d = index % (b+size);

                        // if index == b {
                        //     b += s + (size % 2)
                        // } else if index == 0 {
                        //     b = 0
                        // }

                        size /= 2;
                    } else {
                        break;
                    }
                    // index = (index + 1) % len;
                    println!("i:{index}, b:{b}");
                }
            }
            positions.push(index);
        }
        println!("{positions:?}")
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

    #[test]
    fn shfl() {
        let size = 10;
        let seed = b"hello world".to_vec();
        for index in 0..size {
            println!("{index}, {}", predictive_shuffle::shfl(size, &seed, index))
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
