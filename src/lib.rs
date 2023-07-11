use sha2::{Sha256, *};
use std::cmp::Ordering::{self, Equal, Greater, Less};

const RADIX: u32 = 2;

include!(concat!(env!("OUT_DIR"), "/byte_lookup.rs"));

pub fn shfl(len: usize, seed: &Vec<u8>, mut index: usize) -> usize {
    let log = (len as f32).log2() as usize;

    let mut bits = vec![];
    for byte in seed {
        if let Some(binary) = BYTES.get(byte) {
            bits.append(&mut parse_binary(binary))
        }
    }

    bits = vec![true, true];

    while !bits.is_empty() {
        let mut size = len;
        let mut base: usize = 0;
        for i in 0..log {
            if let Some(bool) = bits.pop() {
                // println!("{size}");

                (index, _) = index_selection(index, size, bool, base);

                size /= 2;

                // println!("i:{index}, b:{base}");

                // if index  >= size && i > 1 && bool {
                //     println!("{size}");
                //     index += size + (size % 2);
                // }
            }
        }
    }

    index
}

pub fn index_selection(
    mut index: usize,
    size: usize,
    bool: bool,
    base: usize,
) -> (usize, Ordering) {
    let mut ord = Equal;

    if bool {
        let diff = size % 2;
        let range = size / 2;

        match &diff {
            0 => match (index - base).cmp(&range) {
                Less => {
                    index += range;
                    ord = Less;
                }
                _ => {
                    index -= range;
                    ord = Greater
                }
            },
            1 => match (index - base).cmp(&range) {
                Less => {
                    index += range + diff;
                    ord = Less
                }
                Equal => ord = Equal,
                Greater => {
                    index -= range + diff;
                    ord = Greater
                }
            },
            _ => {}
        }
    }

    (index, ord)
}

pub fn shuffle(len: usize, seed: &Vec<u8>, mut index: usize) -> usize {
    let log = (len as f32).log2() as usize;

    let mut bits = vec![];
    for byte in seed {
        if let Some(binary) = BYTES.get(byte) {
            bits.append(&mut parse_binary(binary))
        }
    }

    while !bits.is_empty() {
        if bits.len() >= log - 1 {
            let split = bits.split_off(log - 1);
            index = algo(index, log, len, bits);
            bits = split;
        } else {
            index = algo(index, log, len, bits);
            break;
        }
    }
    index
}

pub fn algo(mut index: usize, log: usize, len: usize, mut bits: Vec<bool>) -> usize {
    let mut left;
    let mut right;

    // while !bits.is_empty() {
    index = (index + 1) % len;
    let mut start = 0;
    let mut size = len;
    while !bits.is_empty() {
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

pub fn hash(seed: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(seed);
    let result = hasher.finalize();
    format!("{:x}", result).as_bytes().to_vec()
}

pub fn parse_binary(binary: &str) -> Vec<bool> {
    let b = binary.trim_start().trim_end();
    b.chars()
        .map(|c| matches!(c.to_digit(RADIX).unwrap(), 1))
        .collect()
}
