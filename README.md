# predictive_shuffle
A predictive shuffling algorithm that allows for the predetermined selection of one or many items from a shuffled vec.

We define *predictive* as the ability for users to define the traits of a vector, with some sub-set of indices, as to return the shuffled positions of those indices. This is a one-time operation that only computes the final shuffled locations of the input indicies.

All algorithms are can handle crpytographic, or non-cryptographic shuffling, with all shuffling implementations derived from an optimized version of Durstenfeld's modern implementation of the Fisher-Yates shuffling algo.
