//! # predictive_shuffle
//! A predictive shuffling algorithm that allows for the predetermined selection of one or many items from a shuffled vec.
//!
//! We define *predictive* as the ability for users to define the traits of a vector, with some sub-set of indices, to return the shuffled positions of those indices. This is a one-time operation that only computes the final shuffled locations of the input indices.
//!
//! All algorithms can handle cryptographic, or non-cryptographic shuffling, with all shuffling implementations derived from an optimized version of Durstenfeld's modern implementation of the Fisher-Yates shuffling algo.

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha2::{Sha256, *};
use std::collections::HashMap;

pub trait Shuffle<T> {
    fn fastrand_shuffle(&mut self);
    fn fastrand_shuffle_from_seed(&mut self, seed: Vec<u8>);
    fn modern_shuffle(&mut self) -> Vec<T>;
    fn modern_shuffle_from_seed(&mut self, seed: Vec<u8>) -> Vec<T>;
    fn crypto_modern_shuffle(&mut self) -> Vec<T>;
    fn crypto_modern_shuffle_from_seed(&mut self, seed: Vec<u8>) -> Vec<T>;
    fn predictive_shuffle(&mut self, positions: Vec<usize>) -> HashMap<usize, usize>;
    fn predictive_shuffle_from_seed(
        &mut self,
        positions: Vec<usize>,
        seed: Vec<u8>,
    ) -> HashMap<usize, usize>;
    fn crypto_predictive_shuffle(&mut self, positions: Vec<usize>) -> HashMap<usize, usize>;
    fn crypto_predictive_shuffle_from_seed(
        &mut self,
        positions: Vec<usize>,
        seed: Vec<u8>,
    ) -> HashMap<usize, usize>;
    fn batch_predictive_shuffle(
        &mut self,
        batch: usize,
        positions: Vec<usize>,
    ) -> HashMap<usize, usize>;
    fn batch_predictive_shuffle_from_seed(
        &mut self,
        batch: usize,
        positions: Vec<usize>,
        seed: Vec<u8>,
    ) -> HashMap<usize, usize>;
    fn crypto_batch_predictive_shuffle(
        &mut self,
        batch: usize,
        positions: Vec<usize>,
    ) -> HashMap<usize, usize>;
    fn crypto_batch_predictive_shuffle_from_seed(
        &mut self,
        batch: usize,
        positions: Vec<usize>,
        seed: Vec<u8>,
    ) -> HashMap<usize, usize>;
}

impl<T> Shuffle<T> for Vec<T> {
    /// Shuffle Vector with ['fastrand::Rng']
    ///
    /// ### Example
    ///
    /// Basic usage:
    /// ```rust
    /// use predictive_shuffle::Shuffle;
    ///
    /// let mut vec: Vec<usize> = (0..10).collect();
    ///
    /// vec.fastrand_shuffle()
    /// ```   
    fn fastrand_shuffle(&mut self) {
        let mut rng: fastrand::Rng = fastrand::Rng::new();
        rng.shuffle(self.as_mut_slice());
    }

    /// Shuffle a given vector from Seed with ['fastrand::Rng']
    ///
    /// ### Example
    ///
    /// Basic usage:
    /// ```rust
    /// use predictive_shuffle::Shuffle;
    ///
    /// let mut vec: Vec<usize> = (0..10).collect();
    /// let seed = b"seed phrase".to_vec();
    /// vec.fastrand_shuffle_from_seed(seed)
    /// ```   
    fn fastrand_shuffle_from_seed(&mut self, seed: Vec<u8>) {
        let mut rng: fastrand::Rng = fastrand::Rng::new();
        let seed = byte_array(&seed);
        let seed_int = u64::from_be_bytes(seed);
        rng.seed(seed_int);
        rng.shuffle(self.as_mut_slice());
    }

    /// Shuffle a given vector with the modern Fisher-Yates Algorithm
    ///
    /// ### Example
    ///
    /// Basic usage:
    /// ```rust
    /// use predictive_shuffle::Shuffle;
    ///
    /// let mut vec: Vec<usize> = (0..10).collect();
    /// let shuffled_vec = vec.modern_shuffle();
    /// ```   
    fn modern_shuffle(&mut self) -> Vec<T> {
        let size = self.len();
        let mut rng = fastrand::Rng::new();

        let mut new_vec = vec![];
        for i in (0..size).rev() {
            let x: usize = rng.usize(0..=i);

            new_vec.push(self.swap_remove(x));
        }
        new_vec
    }

    /// Implementing a crpytographic randomization algorithm ['rand_chacha::ChaCha20Rng'],
    /// to shuffle a given vector with the modern Fisher-Yates Algorithm
    ///
    /// ### Example
    ///
    /// Basic usage:
    /// ```rust
    /// use predictive_shuffle::Shuffle;
    ///
    /// let mut vec: Vec<usize> = (0..10).collect();
    /// let shuffled_vec = vec.crypto_modern_shuffle();
    /// ```   
    fn crypto_modern_shuffle(&mut self) -> Vec<T> {
        let size = self.len();
        let mut rng = ChaCha20Rng::from_entropy();

        let mut new_vec = vec![];
        for i in (0..size).rev() {
            let x: usize = rng.gen_range(0..=i);

            new_vec.push(self.swap_remove(x));
        }
        new_vec
    }

    /// Shuffle Vector from Seed with the modern Fisher-Yates Algorithm
    ///
    /// ### Example
    ///
    /// Basic usage:
    /// ```rust
    /// use predictive_shuffle::Shuffle;
    ///
    /// let mut vec: Vec<usize> = (0..10).collect();
    /// let seed = b"seed phrase".to_vec();
    /// let shuffled_vec = vec.modern_shuffle_from_seed(seed);
    /// ```   
    fn modern_shuffle_from_seed(&mut self, seed: Vec<u8>) -> Vec<T> {
        let size = self.len();
        let seed = byte_array(&seed);

        let seed_int = u64::from_be_bytes(seed);

        let mut rng = fastrand::Rng::new();
        rng.seed(seed_int);

        let mut new_vec = vec![];
        for i in (0..size).rev() {
            let x: usize = rng.usize(0..=i);

            new_vec.push(self.swap_remove(x));
        }
        new_vec
    }

    /// Implementing a crpytographic rnadomization algorithm ['rand_chacha::ChaCha20Rng'],
    /// shuffle a given vector from with a with the modern Fisher-Yates Algorithm
    ///
    /// ### Example
    ///
    /// Basic usage:
    /// ```rust
    /// use predictive_shuffle::Shuffle;
    ///
    /// let mut vec: Vec<usize> = (0..10).collect();
    /// let seed = b"seed phrase".to_vec();
    /// let shuffled_vec = vec.crypto_modern_shuffle_from_seed(seed);
    /// ```  
    fn crypto_modern_shuffle_from_seed(&mut self, seed: Vec<u8>) -> Vec<T> {
        let size = self.len();
        let seed = byte_array(&seed);

        let seed_int = u64::from_be_bytes(seed);

        let mut rng = ChaCha20Rng::seed_from_u64(seed_int);

        let mut new_vec = vec![];
        for i in (0..size).rev() {
            let x: usize = rng.gen_range(0..=i);

            new_vec.push(self.swap_remove(x));
        }
        new_vec
    }

    /// Predict Shuffled Position of Items
    ///
    /// ### Example
    ///
    /// Basic usage:
    /// ```rust
    /// use predictive_shuffle::Shuffle;
    ///
    /// let mut vec: Vec<usize> = (0..10).collect();
    /// let seed = b"seed phrase".to_vec();
    /// let positions = vec![1,5];
    /// let shuffled_vec = vec.predictive_shuffle(positions);
    /// ```  
    fn predictive_shuffle(&mut self, positions: Vec<usize>) -> HashMap<usize, usize> {
        let size = self.len();
        // random function
        let mut rand: fastrand::Rng = fastrand::Rng::new();

        // mutable structures
        let mut vec: Vec<Option<usize>> = vec![None; self.len()];
        let mut peers = positions.len();

        // fill vec
        for i in positions {
            vec[i] = Some(i)
        }
        let mut new_map = HashMap::new();
        for i in (0..size).rev() {
            let x: usize = rand.usize(0..=i);

            if let Some(item) = vec[x] {
                new_map.insert(item, i);
                peers -= 1;
                vec[x] = None;
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

    /// Predict Shuffled Position of Items from Seed
    ///
    /// ### Example
    ///
    /// Basic usage:
    /// ```rust
    /// use predictive_shuffle::Shuffle;
    ///
    /// let mut vec: Vec<usize> = (0..100).collect();
    /// let seed = b"seed phrase".to_vec();
    /// let positions = vec![1,5];
    /// let new_positions = vec.predictive_shuffle_from_seed(positions, seed);
    /// assert_eq!(new_positions.get(&1), Some(&79));
    /// assert_eq!(new_positions.get(&5), Some(&86));
    /// ```  
    fn predictive_shuffle_from_seed(
        &mut self,
        positions: Vec<usize>,
        seed: Vec<u8>,
    ) -> HashMap<usize, usize> {
        let size = self.len();
        // seed
        let seed = byte_array(&seed);
        let seed_int = u64::from_be_bytes(seed);

        // random function
        let mut rand: fastrand::Rng = fastrand::Rng::new();
        rand.seed(seed_int);

        // mutable structures
        let mut vec: Vec<Option<usize>> = vec![None; self.len()];
        let mut peers = positions.len();

        // fill vec
        for i in positions {
            vec[i] = Some(i)
        }
        let mut new_map = HashMap::new();
        for i in (0..size).rev() {
            let x: usize = rand.usize(0..=i);

            if let Some(item) = vec[x] {
                new_map.insert(item, i);
                peers -= 1;
                vec[x] = None;
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

    /// Predict Shuffled Position of Items
    ///
    /// ### Example
    ///
    /// Basic usage:
    /// ```rust
    /// use predictive_shuffle::Shuffle;
    ///
    /// let mut vec: Vec<usize> = (0..10).collect();
    /// let seed = b"seed phrase".to_vec();
    /// let positions = vec![1,5];
    /// let shuffled_vec = vec.crypto_predictive_shuffle(positions);
    /// ```  
    fn crypto_predictive_shuffle(&mut self, positions: Vec<usize>) -> HashMap<usize, usize> {
        let size = self.len();
        // random function
        let mut rng = ChaCha20Rng::from_entropy();

        // mutable structures
        let mut vec: Vec<Option<usize>> = vec![None; self.len()];
        let mut peers = positions.len();

        // fill vec
        for i in positions {
            vec[i] = Some(i)
        }
        let mut new_map = HashMap::new();
        for i in (0..size).rev() {
            let x: usize = rng.gen_range(0..=i);

            if let Some(item) = vec[x] {
                new_map.insert(item, i);
                peers -= 1;
                vec[x] = None;
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

    /// Predict Shuffled Position of Items from Seed
    ///
    /// ### Example
    ///
    /// Basic usage:
    /// ```rust
    /// use predictive_shuffle::Shuffle;
    ///
    /// let mut vec: Vec<usize> = (0..100).collect();
    /// let seed = b"seed phrase".to_vec();
    /// let positions = vec![1,5];
    /// let new_positions = vec.crypto_predictive_shuffle_from_seed(positions, seed);
    /// assert_eq!(new_positions.get(&1), Some(&8));
    /// assert_eq!(new_positions.get(&5), Some(&91));
    /// ```  
    fn crypto_predictive_shuffle_from_seed(
        &mut self,
        positions: Vec<usize>,
        seed: Vec<u8>,
    ) -> HashMap<usize, usize> {
        let size = self.len();
        // seed
        let seed = byte_array(&seed);
        let seed_int = u64::from_be_bytes(seed);

        // random function
        let mut rng = ChaCha20Rng::seed_from_u64(seed_int);

        // mutable structures
        let mut vec: Vec<Option<usize>> = vec![None; self.len()];
        let mut peers = positions.len();

        // fill vec
        for i in positions {
            vec[i] = Some(i)
        }

        let mut new_map = HashMap::new();
        for i in (0..size).rev() {
            let x: usize = rng.gen_range(0..i);

            if let Some(item) = vec[x] {
                new_map.insert(item, i);
                peers -= 1;
                vec[x] = None;
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

    /// Predict Shuffled Position of Items from Seed
    ///
    /// ### Example
    ///
    /// Basic usage:
    /// ```rust
    /// use predictive_shuffle::Shuffle;
    ///
    /// let mut vec: Vec<usize> = (0..100).collect();
    /// let batch = 4;
    /// let positions = vec![1,5];
    /// let new_positions = vec.batch_predictive_shuffle(batch, positions);
    /// ```  
    fn batch_predictive_shuffle(
        &mut self,
        batch: usize,
        positions: Vec<usize>,
    ) -> HashMap<usize, usize> {
        let size = self.len();

        // random function
        let mut rand: fastrand::Rng = fastrand::Rng::new();

        // mutable structures
        let mut vec: Vec<Option<usize>> = vec![None; self.len()];
        let mut peers = positions.len();

        // fill vec
        for i in positions {
            vec[i] = Some(i)
        }

        // iterate over all items
        let mut new_map = HashMap::new();
        let range = size / batch;
        let mut randoms: Vec<usize> = (0..range).collect();
        rand.shuffle(randoms.as_mut_slice());
        for i in (0..size).rev() {
            let x: usize = if i == 0 {
                0
            } else {
                randoms[i % randoms.len()] % i
            };

            if let Some(item) = vec[x] {
                new_map.insert(item, i);
                peers -= 1;
                vec[x] = None;
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

    /// Predict Shuffled Position of Items from Seed
    ///
    /// ### Example
    ///
    /// Basic usage:
    /// ```rust
    /// use predictive_shuffle::Shuffle;
    ///
    /// let mut vec: Vec<usize> = (0..100).collect();
    /// let batch = 4;
    /// let seed = b"seed phrase".to_vec();
    /// let positions = vec![1,5];
    /// let new_positions = vec.batch_predictive_shuffle_from_seed(batch, positions, seed);
    /// assert_eq!(new_positions.get(&1), Some(&96));
    /// assert_eq!(new_positions.get(&5), Some(&90));
    /// ```  
    fn batch_predictive_shuffle_from_seed(
        &mut self,
        batch: usize,
        positions: Vec<usize>,
        seed: Vec<u8>,
    ) -> HashMap<usize, usize> {
        let size = self.len();

        // seed
        let seed = byte_array(&seed);
        let seed_int = u64::from_be_bytes(seed);

        // random function
        let mut rand: fastrand::Rng = fastrand::Rng::new();
        rand.seed(seed_int);

        // mutable structures
        let mut vec: Vec<Option<usize>> = vec![None; self.len()];
        let mut peers = positions.len();

        // fill vec
        for i in positions {
            vec[i] = Some(i)
        }

        // iterate over all items
        let mut new_map = HashMap::new();
        let range = size / batch;
        let mut randoms: Vec<usize> = (0..range).collect();
        rand.shuffle(randoms.as_mut_slice());
        for i in (0..size).rev() {
            let x: usize = if i == 0 {
                0
            } else {
                randoms[i % randoms.len()] % i
            };

            if let Some(item) = vec[x] {
                new_map.insert(item, i);
                peers -= 1;
                vec[x] = None;
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

    /// Predict Shuffled Position of Items from Seed
    ///
    /// ### Example
    ///
    /// Basic usage:
    /// ```rust
    /// use predictive_shuffle::Shuffle;
    ///
    /// let mut vec: Vec<usize> = (0..100).collect();
    /// let batch = 4;
    /// let positions = vec![1,5];
    /// let new_positions = vec.crypto_batch_predictive_shuffle(batch, positions);
    /// ```  
    fn crypto_batch_predictive_shuffle(
        &mut self,
        batch: usize,
        positions: Vec<usize>,
    ) -> HashMap<usize, usize> {
        let size = self.len();

        // random function
        let mut rng = ChaCha20Rng::from_entropy();

        // mutable structures
        let mut vec: Vec<Option<usize>> = vec![None; self.len()];
        let mut peers = positions.len();

        // fill vec
        for i in positions {
            vec[i] = Some(i)
        }

        // iterate over all items
        let mut new_map = HashMap::new();
        let range = size / batch;
        let mut randoms: Vec<usize> = vec![];
        for i in (0..size).rev() {
            let x: usize = if i > size - range {
                let x = rng.gen_range(0..=i);
                randoms.push(x);
                x
            } else if i == 0 {
                0
            } else {
                randoms[i % randoms.len()] % i
            };

            if let Some(item) = vec[x] {
                new_map.insert(item, i);
                peers -= 1;
                vec[x] = None;
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

    /// Predict Shuffled Position of Items from Seed
    ///
    /// ### Example
    ///
    /// Basic usage:
    /// ```rust
    /// use predictive_shuffle::Shuffle;
    ///
    /// let mut vec: Vec<usize> = (0..100).collect();
    /// let batch = 4;
    /// let seed = b"seed phrase".to_vec();
    /// let positions = vec![1,5];
    /// let new_positions = vec.crypto_batch_predictive_shuffle_from_seed(batch, positions, seed);
    /// assert_eq!(new_positions.get(&1), Some(&7));
    /// assert_eq!(new_positions.get(&5), Some(&93));
    /// ```  
    fn crypto_batch_predictive_shuffle_from_seed(
        &mut self,
        batch: usize,
        positions: Vec<usize>,
        seed: Vec<u8>,
    ) -> HashMap<usize, usize> {
        let size = self.len();

        // seed
        let seed = byte_array(&seed);
        let seed_int = u64::from_be_bytes(seed);

        // random function
        let mut rng = ChaCha20Rng::seed_from_u64(seed_int);

        // mutable structures
        let mut vec: Vec<Option<usize>> = vec![None; self.len()];
        let mut peers = positions.len();

        // fill vec
        for i in positions {
            vec[i] = Some(i)
        }

        // iterate over all items
        let mut new_map = HashMap::new();
        let range = size / batch;
        let mut randoms: Vec<usize> = vec![];
        for i in (0..size).rev() {
            let x: usize = if i > size - range {
                let x = rng.gen_range(0..=i);
                randoms.push(x);
                x
            } else if i == 0 {
                0
            } else {
                randoms[i % randoms.len()] % i
            };

            if let Some(item) = vec[x] {
                new_map.insert(item, i);
                peers -= 1;
                vec[x] = None;
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
}

/// Generate shuffled vector with ['fastrand::Rng']
///
/// ### Example
///
/// Basic usage:
/// ```rust
/// use predictive_shuffle::gen_fastrand_shuffle;
///
/// let size = 10;
///
/// let shuffled_vec = gen_fastrand_shuffle(size);
/// ```   
pub fn gen_fastrand_shuffle(size: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = (0..size).collect();
    let mut rng: fastrand::Rng = fastrand::Rng::new();
    rng.shuffle(vec.as_mut_slice());
    vec
}

/// Generate shuffled vector with from a seed with ['fastrand::Rng']
///
/// ### Example
///
/// Basic usage:
/// ```rust
/// use predictive_shuffle::gen_fastrand_shuffle_from_seed;
///
/// let size = 10;
/// let seed = b"seed phrase".to_vec();
/// let shuffled_vec = gen_fastrand_shuffle_from_seed(size, seed);
/// ```   
pub fn gen_fastrand_shuffle_from_seed(size: usize, seed: Vec<u8>) -> Vec<usize> {
    let mut vec: Vec<usize> = (0..size).collect();
    let mut rng: fastrand::Rng = fastrand::Rng::new();
    let seed = byte_array(&seed);
    let seed_int = u64::from_be_bytes(seed);
    rng.seed(seed_int);
    rng.shuffle(vec.as_mut_slice());
    vec
}

/// Generate a shuffled vector with the modern Fisher-Yates Algorithm
///
/// ### Example
///
/// Basic usage:
/// ```rust
/// use predictive_shuffle::gen_modern_shuffle;
///
/// let size = 10;
/// let shuffled_vec = gen_modern_shuffle(10);
/// ```   
pub fn gen_modern_shuffle(size: usize) -> Vec<usize> {
    let mut rng = fastrand::Rng::new();
    let mut vec: Vec<usize> = (0..size).collect();
    let mut new_vec = vec![];
    for i in (0..size).rev() {
        let x: usize = rng.usize(0..=i);

        new_vec.push(vec.swap_remove(x));
    }
    new_vec
}

/// Implementing a crpytographic randomization algorithm ['rand_chacha::ChaCha20Rng'],
/// to generate a shuffled vector with the modern Fisher-Yates Algorithm
///
/// ### Example
///
/// Basic usage:
/// ```rust
/// use predictive_shuffle::gen_crypto_modern_shuffle;
///
/// let size = 10;
/// let shuffled_vec = gen_crypto_modern_shuffle(size);
/// ```   
pub fn gen_crypto_modern_shuffle(size: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = (0..size).collect();
    let mut rng = ChaCha20Rng::from_entropy();

    let mut new_vec = vec![];
    for i in (0..size).rev() {
        let x: usize = rng.gen_range(0..=i);

        new_vec.push(vec.swap_remove(x));
    }
    new_vec
}

/// Generate a shuffled vector from a seed with the modern Fisher-Yates Algorithm
///
/// ### Example
///
/// Basic usage:
/// ```rust
/// use predictive_shuffle::gen_modern_shuffle_from_seed;
///
/// let size = 10;
/// let seed = b"seed phrase".to_vec();
/// let shuffled_vec = gen_modern_shuffle_from_seed(size, seed);
/// ```   
pub fn gen_modern_shuffle_from_seed(size: usize, seed: Vec<u8>) -> Vec<usize> {
    let mut vec: Vec<usize> = (0..size).collect();
    let seed = byte_array(&seed);

    let seed_int = u64::from_be_bytes(seed);

    let mut rng = fastrand::Rng::new();
    rng.seed(seed_int);

    let mut new_vec = vec![];
    for i in (0..size).rev() {
        let x: usize = rng.usize(0..=i);

        new_vec.push(vec.swap_remove(x));
    }
    new_vec
}

/// Implementing a crpytographic rnadomization algorithm ['rand_chacha::ChaCha20Rng'],
/// shuffle a given vector from with a with the modern Fisher-Yates Algorithm
///
/// ### Example
///
/// Basic usage:
/// ```rust
/// use predictive_shuffle::gen_crypto_modern_shuffle_from_seed;
///
/// let size = 10;
/// let seed = b"seed phrase".to_vec();
/// let shuffled_vec = gen_crypto_modern_shuffle_from_seed(size, seed);
/// ```  
pub fn gen_crypto_modern_shuffle_from_seed(size: usize, seed: Vec<u8>) -> Vec<usize> {
    let mut vec: Vec<usize> = (0..10).collect();
    let seed = byte_array(&seed);

    let seed_int = u64::from_be_bytes(seed);

    let mut rng = ChaCha20Rng::seed_from_u64(seed_int);

    let mut new_vec = vec![];
    for i in (0..size).rev() {
        let x: usize = rng.gen_range(0..=i);

        new_vec.push(vec.swap_remove(x));
    }
    new_vec
}

/// Predict shuffled position from size and starting position(s)
///
/// ### Example
///
/// Basic usage:
/// ```rust
/// use predictive_shuffle::gen_predictive_shuffle;
///
/// let size = 100;
/// let seed = b"seed phrase".to_vec();
/// let positions = vec![1,5];
/// let shuffled_vec = gen_predictive_shuffle(size, positions);
/// ```  
pub fn gen_predictive_shuffle(size: usize, positions: Vec<usize>) -> HashMap<usize, usize> {
    // random function
    let mut rand: fastrand::Rng = fastrand::Rng::new();

    // mutable structures
    let mut vec: Vec<Option<usize>> = vec![None; size];
    let mut peers = positions.len();

    // fill vec
    for i in positions {
        vec[i] = Some(i)
    }
    let mut new_map = HashMap::new();
    for i in (0..size).rev() {
        let x: usize = rand.usize(0..=i);

        if let Some(item) = vec[x] {
            new_map.insert(item, i);
            peers -= 1;
            vec[x] = None;
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

/// Predict shuffled position from size and starting position(s) with a seed
///
/// ### Example
///
/// Basic usage:
/// ```rust
/// use predictive_shuffle::gen_predictive_shuffle_from_seed;
///
/// let size = 100;
/// let seed = b"seed phrase".to_vec();
/// let positions = vec![1,5];
/// let new_positions = gen_predictive_shuffle_from_seed(size, positions, seed);
/// assert_eq!(new_positions.get(&1), Some(&79));
/// assert_eq!(new_positions.get(&5), Some(&86));
/// ```  
pub fn gen_predictive_shuffle_from_seed(
    size: usize,
    positions: Vec<usize>,
    seed: Vec<u8>,
) -> HashMap<usize, usize> {
    // seed
    let seed = byte_array(&seed);
    let seed_int = u64::from_be_bytes(seed);

    // random function
    let mut rand: fastrand::Rng = fastrand::Rng::new();
    rand.seed(seed_int);

    // mutable structures
    let mut vec: Vec<Option<usize>> = vec![None; size];
    let mut peers = positions.len();

    // fill vec
    for i in positions {
        vec[i] = Some(i)
    }
    let mut new_map = HashMap::new();
    for i in (0..size).rev() {
        let x: usize = rand.usize(0..=i);

        if let Some(item) = vec[x] {
            new_map.insert(item, i);
            peers -= 1;
            vec[x] = None;
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

/// Predict Shuffled Position of Items
///
/// ### Example
///
/// Basic usage:
/// ```rust
/// use predictive_shuffle::gen_crypto_predictive_shuffle;
///
/// let size = 100;
/// let seed = b"seed phrase".to_vec();
/// let positions = vec![1,5];
/// let shuffled_vec = gen_crypto_predictive_shuffle(size, positions);
/// ```  
pub fn gen_crypto_predictive_shuffle(size: usize, positions: Vec<usize>) -> HashMap<usize, usize> {
    // random function
    let mut rng = ChaCha20Rng::from_entropy();

    // mutable structures
    let mut vec: Vec<Option<usize>> = vec![None; size];
    let mut peers = positions.len();

    // fill vec
    for i in positions {
        vec[i] = Some(i)
    }
    let mut new_map = HashMap::new();
    for i in (0..size).rev() {
        let x: usize = rng.gen_range(0..=i);

        if let Some(item) = vec[x] {
            new_map.insert(item, i);
            peers -= 1;
            vec[x] = None;
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

/// Predict Shuffled Position of Items from Seed
///
/// ### Example
///
/// Basic usage:
/// ```rust
/// use predictive_shuffle::gen_crypto_predictive_shuffle_from_seed;
///
/// let size = 100;
/// let seed = b"seed phrase".to_vec();
/// let positions = vec![1,5];
/// let new_positions = gen_crypto_predictive_shuffle_from_seed(size, positions, seed);
/// assert_eq!(new_positions.get(&1), Some(&8));
/// assert_eq!(new_positions.get(&5), Some(&91));
/// ```  
pub fn gen_crypto_predictive_shuffle_from_seed(
    size: usize,
    positions: Vec<usize>,
    seed: Vec<u8>,
) -> HashMap<usize, usize> {
    // seed
    let seed = byte_array(&seed);
    let seed_int = u64::from_be_bytes(seed);

    // random function
    let mut rng = ChaCha20Rng::seed_from_u64(seed_int);

    // mutable structures
    let mut vec: Vec<Option<usize>> = vec![None; size];
    let mut peers = positions.len();

    // fill vec
    for i in positions {
        vec[i] = Some(i)
    }

    let mut new_map = HashMap::new();
    for i in (0..size).rev() {
        let x: usize = rng.gen_range(0..i);

        if let Some(item) = vec[x] {
            new_map.insert(item, i);
            peers -= 1;
            vec[x] = None;
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

/// Predict Shuffled Position of Items from Seed
///
/// ### Example
///
/// Basic usage:
/// ```rust
/// use predictive_shuffle::gen_batch_predictive_shuffle;
///
/// let size = 100;
/// let batch = 4;
/// let positions = vec![1,5];
/// let new_positions = gen_batch_predictive_shuffle(size, batch, positions);
/// ```  
pub fn gen_batch_predictive_shuffle(
    size: usize,
    batch: usize,
    positions: Vec<usize>,
) -> HashMap<usize, usize> {
    // random function
    let mut rand: fastrand::Rng = fastrand::Rng::new();

    // mutable structures
    let mut vec: Vec<Option<usize>> = vec![None; size];
    let mut peers = positions.len();

    // fill vec
    for i in positions {
        vec[i] = Some(i)
    }

    // iterate over all items
    let mut new_map = HashMap::new();
    let range = size / batch;
    let mut randoms: Vec<usize> = (0..range).collect();
    rand.shuffle(randoms.as_mut_slice());
    for i in (0..size).rev() {
        let x: usize = if i == 0 {
            0
        } else {
            randoms[i % randoms.len()] % i
        };

        if let Some(item) = vec[x] {
            new_map.insert(item, i);
            peers -= 1;
            vec[x] = None;
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

/// Predict Shuffled Position of Items from Seed
///
/// ### Example
///
/// Basic usage:
/// ```rust
/// use predictive_shuffle::gen_batch_predictive_shuffle_from_seed;
///
/// let size = 100;
/// let batch = 4;
/// let seed = b"seed phrase".to_vec();
/// let positions = vec![1,5];
/// let new_positions = gen_batch_predictive_shuffle_from_seed(size, batch, positions, seed);
/// assert_eq!(new_positions.get(&1), Some(&96));
/// assert_eq!(new_positions.get(&5), Some(&90));
/// ```  
pub fn gen_batch_predictive_shuffle_from_seed(
    size: usize,
    batch: usize,
    positions: Vec<usize>,
    seed: Vec<u8>,
) -> HashMap<usize, usize> {
    // seed
    let seed = byte_array(&seed);
    let seed_int = u64::from_be_bytes(seed);

    // random function
    let mut rand: fastrand::Rng = fastrand::Rng::new();
    rand.seed(seed_int);

    // mutable structures
    let mut vec: Vec<Option<usize>> = vec![None; size];
    let mut peers = positions.len();

    // fill vec
    for i in positions {
        vec[i] = Some(i)
    }

    // iterate over all items
    let mut new_map = HashMap::new();
    let range = size / batch;
    let mut randoms: Vec<usize> = (0..range).collect();
    rand.shuffle(randoms.as_mut_slice());
    for i in (0..size).rev() {
        let x: usize = if i == 0 {
            0
        } else {
            randoms[i % randoms.len()] % i
        };

        if let Some(item) = vec[x] {
            new_map.insert(item, i);
            peers -= 1;
            vec[x] = None;
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

/// Predict Shuffled Position of Items from Seed
///
/// ### Example
///
/// Basic usage:
/// ```rust
/// use predictive_shuffle::gen_crypto_batch_predictive_shuffle;
///
/// let size = 100;
/// let batch = 4;
/// let positions = vec![1,5];
/// let new_positions = gen_crypto_batch_predictive_shuffle(size, batch, positions);
/// ```  
pub fn gen_crypto_batch_predictive_shuffle(
    size: usize,
    batch: usize,
    positions: Vec<usize>,
) -> HashMap<usize, usize> {
    // random function
    let mut rng = ChaCha20Rng::from_entropy();

    // mutable structures
    let mut vec: Vec<Option<usize>> = vec![None; size];
    let mut peers = positions.len();

    // fill vec
    for i in positions {
        vec[i] = Some(i)
    }

    // iterate over all items
    let mut new_map = HashMap::new();
    let range = size / batch;
    let mut randoms: Vec<usize> = vec![];
    for i in (0..size).rev() {
        let x: usize = if i > size - range {
            let x = rng.gen_range(0..=i);
            randoms.push(x);
            x
        } else if i == 0 {
            0
        } else {
            randoms[i % randoms.len()] % i
        };

        if let Some(item) = vec[x] {
            new_map.insert(item, i);
            peers -= 1;
            vec[x] = None;
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

/// Predict Shuffled Position of Items from Seed
///
/// ### Example
///
/// Basic usage:
/// ```rust
/// use predictive_shuffle::gen_crypto_batch_predictive_shuffle_from_seed;
///
/// let size = 100;
/// let batch = 4;
/// let seed = b"seed phrase".to_vec();
/// let positions = vec![1,5];
/// let new_positions = gen_crypto_batch_predictive_shuffle_from_seed(size, batch, positions, seed);
/// assert_eq!(new_positions.get(&1), Some(&7));
/// assert_eq!(new_positions.get(&5), Some(&93));
/// ```  
pub fn gen_crypto_batch_predictive_shuffle_from_seed(
    size: usize,
    batch: usize,
    positions: Vec<usize>,
    seed: Vec<u8>,
) -> HashMap<usize, usize> {
    // seed
    let seed = byte_array(&seed);
    let seed_int = u64::from_be_bytes(seed);

    // random function
    let mut rng = ChaCha20Rng::seed_from_u64(seed_int);

    // mutable structures
    let mut vec: Vec<Option<usize>> = vec![None; size];
    let mut peers = positions.len();

    // fill vec
    for i in positions {
        vec[i] = Some(i)
    }

    // iterate over all items
    let mut new_map = HashMap::new();
    let range = size / batch;
    let mut randoms: Vec<usize> = vec![];
    for i in (0..size).rev() {
        let x: usize = if i > size - range {
            let x = rng.gen_range(0..=i);
            randoms.push(x);
            x
        } else if i == 0 {
            0
        } else {
            randoms[i % randoms.len()] % i
        };

        if let Some(item) = vec[x] {
            new_map.insert(item, i);
            peers -= 1;
            vec[x] = None;
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
