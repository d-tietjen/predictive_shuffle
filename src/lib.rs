use std::collections::HashMap;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use sha2::{Sha256, *};

pub fn multi_index_shuffle_prediction(
    ids: &Vec<usize>,
    seed: &Vec<u8>,
    size: usize,
) -> HashMap<usize, usize> {
    // let mut map = HashMap::new();
    // for index in ids {
    //     map.insert(*index, *index);
    // }

    let mut hash = hash(seed);
    hash.truncate(8);
    let seed = demo(hash);

    let mut arr = [0u8; 8];
    arr.copy_from_slice(&seed);
    let seed_int = u64::from_be_bytes(arr);

    let mut base1 = fastrand::Rng::new();
    base1.seed(seed_int);

    // let mut rng = fastrand::seed(seed_int);

    let mut new_map = HashMap::new();

    let mut vec: Vec<Option<usize>> = vec![None; size];
    let mut peers = ids.len();

    for i in (0..size).rev() {
        let x: usize = base1.usize(0..=i);

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
        // if let Some((_index, origin)) = map.remove_entry(&x) {
        //     new_map.insert(origin, i);
        //     if map.is_empty() {
        //         break;
        //     }
        // }
        // if let Some((_index, origin)) = map.remove_entry(&i) {
        //     map.insert(x, origin);
        // }
    }

    new_map
}

// pub fn shuffle_prediction(mut index: usize, seed: &Vec<u8>, size: usize) -> usize {
//     let mut hash = hash(seed);
//     hash.truncate(32);
//     let seed = demo(hash);

//     let mut rng = ChaCha8Rng::from_seed(seed);

//     for i in (0..size).rev() {
//         let x: usize = rng.gen_range(0..(i + 1));

//         if x == index {
//             index = i;
//             break;
//         } else if i == index {
//             index = x
//         };
//     }
//     index
// }

// pub fn worst_case(mut index: usize, seed: &Vec<u8>, size: usize) -> usize {
//     let mut hash = hash(seed);
//     hash.truncate(32);
//     let seed = demo(hash);

//     let mut rng = ChaCha8Rng::from_seed(seed);

//     for i in (0..size).rev() {
//         let x: usize = rng.gen_range(0..(i + 1));

//         if x == index {
//             index = i;
//         } else if i == index {
//             index = x
//         };
//     }
//     index
// }

pub fn hash(seed: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(seed);
    let result = hasher.finalize();
    format!("{:x}", result).as_bytes().to_vec()
}

pub fn demo(v: Vec<u8>) -> [u8; 8] {
    v.try_into().unwrap_or_else(|v: Vec<u8>| {
        panic!("Expected a Vec of length {} but it was {}", 8, v.len())
    })
}

pub fn shuffle_vec(mut vec: Vec<usize>, seed: &Vec<u8>, size: usize) -> Vec<usize> {
    let mut hash = hash(seed);
    hash.truncate(8);
    let seed = demo(hash);

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
