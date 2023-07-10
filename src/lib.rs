const BASE: usize = 2;

const RADIX: u32 = 2;

include!(concat!(env!("OUT_DIR"), "/byte_lookup.rs"));

pub fn shuffle(len: usize, seed: &Vec<u8>, mut index: usize) -> usize {
    // seed = hash(seed);

    let log = (len as f32).log2() as usize;

    // XOR seed bytes down to some number of bits
    // for _ in 0..=1 {
    //     let v2 = seed.split_off(seed.len() / 2);
    //     seed.iter_mut()
    //         .zip(v2.iter())
    //         .for_each(|(x1, x2)| *x1 ^= *x2);
    // }

    // seed.iter().map(|b| _b).collect();
    let mut bits = vec![];
    for byte in seed {
        if let Some(binary) = BYTES.get(byte) {
            bits.append(&mut parse_binary(binary))
        }
        // for i in 0..8 {
        //     let mask = 1 << i;
        //     let bit = (mask & byte) > 0;
        //     bits.push(bit)
        // }
    }

    // bits.truncate(5);

    // println!("{bits:?}");

    let mut left;
    let mut right;

    // while !bits.is_empty() {
    index = (index + 1) % len;
    let mut start = 0;
    let mut size = len;
    for _ in 1..log {
        if let Some(bool) = bits.pop() {
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

            // println!("Index: {index}, Start: l{start} {left}, {right}");
        } else {
            break;
        }
    }
    // }
    index
}

// fn hash(seed: Vec<u8>) -> Vec<u8> {
//     let mut hasher = Sha256::new();
//     hasher.update(seed);
//     let result = hasher.finalize();
//     format!("{:x}", result).as_bytes().to_vec()
// }

pub fn parse_binary(binary: &str) -> Vec<bool> {
    let b = binary.trim_start().trim_end();
    b.chars()
        .map(|c| matches!(c.to_digit(RADIX).unwrap(), 1))
        .collect()
}
