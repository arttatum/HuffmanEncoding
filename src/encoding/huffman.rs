use bitvec::vec::BitVec;
pub mod encoder;
pub mod tree;

pub use encoder::HuffmanEncoder;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use tree::HuffmanTree;

pub fn compress<T: Hash + Eq + Clone + Display, I: Iterator<Item = T>>(
    token_iterator: I,
    frequencies: HashMap<T, u32>,
) -> CompressedData<T> {
    let huffman_tree = HuffmanTree::from_frequencies(&frequencies);
    let encoder = HuffmanEncoder::from_huffman_tree(huffman_tree);
    CompressedData {
        decoder: encoder.decoder.clone(),
        data: encoder.encode(token_iterator),
    }
}

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct CompressedData<T> {
    pub data: BitVec,
    pub decoder: HashMap<BitVec, T>,
}
