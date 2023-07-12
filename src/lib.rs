use sha2::{Sha256, *};
use std::collections::HashMap;

pub fn multi_index_shuffle_prediction(
    ids: &Vec<usize>,
    seed: &Vec<u8>,
    size: usize,
) -> HashMap<usize, usize> {
    // seed
    let seed = byte_array(seed);
    let seed_int = u64::from_be_bytes(seed);

    // random function
    let mut rand: fastrand::Rng = fastrand::Rng::new();
    rand.seed(seed_int);

    // mutable structures
    let mut vec: Vec<Option<usize>> = vec![None; size];
    let peers = ids.len();

    // fill vec
    for i in ids {
        vec[*i] = Some(*i)
    }

    complete_search(size, rand, peers, vec)
}

pub fn skip_multi_index_shuffle_prediction(
    ids: &Vec<usize>,
    seed: &Vec<u8>,
    size: usize,
    randomness: f32,
) -> HashMap<usize, usize> {
    // seed
    let seed = byte_array(seed);
    let seed_int = u64::from_be_bytes(seed);

    // random function
    let mut rand: fastrand::Rng = fastrand::Rng::new();
    rand.seed(seed_int);

    // mutable structures
    let mut vec: Vec<Option<usize>> = vec![None; size];
    let peers = ids.len();

    // fill vec
    for i in ids {
        vec[*i] = Some(*i)
    }

    skip_search(size, randomness, rand, peers, vec)
}

pub fn complete_search(
    size: usize,
    mut rand: fastrand::Rng,
    mut peers: usize,
    mut vec: Vec<Option<usize>>,
) -> HashMap<usize, usize> {
    // iterate over all items
    let mut new_map = HashMap::new();
    for i in (0..size).rev() {
        let x: usize = rand.usize(0..=i);

        if let Some(item) = vec[x] {
            new_map.insert(item, i);
            peers -= 1;
            if peers == 0 {
                break;
            }
        }

        if let Some(item) = vec[i] {
            vec[x] = Some(item);
            vec[i] = None
        }
    }
    new_map
}

pub fn skip_search(
    size: usize,
    batch: f32,
    mut rand: fastrand::Rng,
    mut peers: usize,
    mut vec: Vec<Option<usize>>,
) -> HashMap<usize, usize> {
    // iterate over all items
    let mut new_map = HashMap::new();
    let range = size - (size as f32 * batch) as usize;
    let mut rand_vec = Vec::with_capacity(range);
    for i in (0..size).rev() {
        let x: usize = if i >= range {
            let x = rand.usize(0..=i);
            rand_vec.push(x);
            x
        } else {
            i % rand_vec[i % rand_vec.len()]
        };

        if let Some(item) = vec[x] {
            new_map.insert(item, i);
            peers -= 1;
            if peers == 0 {
                break;
            }
        }

        if let Some(item) = vec[i] {
            vec[x] = Some(item);
            vec[i] = None
        }
    }
    new_map
}

pub fn hash(seed: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(seed);
    let result = hasher.finalize();
    format!("{:x}", result).as_bytes().to_vec()
}

pub fn byte_array(v: &Vec<u8>) -> [u8; 8] {
    let mut hash = hash(v);
    hash.truncate(8);
    hash.try_into().unwrap_or_else(|v: Vec<u8>| {
        panic!("Expected a Vec of length {} but it was {}", 8, v.len())
    })
}

pub fn shuffle_vec(mut vec: Vec<usize>, seed: &Vec<u8>, size: usize) -> Vec<usize> {
    let seed = byte_array(seed);

    let seed_int = u64::from_be_bytes(seed);

    let mut rand = fastrand::Rng::new();
    rand.seed(seed_int);

    let mut new_vec = vec![];
    for i in (0..size).rev() {
        let x: usize = rand.usize(0..=i);

        new_vec.push(vec.swap_remove(x));
    }
    new_vec
}
