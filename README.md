## predictive_shuffle
A predictive shuffling algorithm that allows for the predetermined selection of one or many items from a shuffled vec.

We define *predictive* as the ability for users to define the traits of a vector and some sub-set of positional indices, as to return the shuffled positions of those indices. This is a one-time operation that only computes the final shuffled locations of the input indices.

All algorithms can handle cryptographic, or non-cryptographic shuffling, with all shuffling implementations derived from an optimized version of the modern implementation of the Fisher-Yates shuffling algo, or other methods defined within the relevant crates.

License: MIT
