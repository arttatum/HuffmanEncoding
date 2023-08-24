use bit_vec::BitVec;
use serde::{Deserialize, Serialize};
pub mod encoder;
pub mod tree;

pub use encoder::HuffmanEncoder;
use rayon::prelude::*;
use std::{collections::HashMap, fmt::Display, hash::Hash};
use tree::HuffmanTree;

pub fn compress<'a, T, TExtractor, TokenIterator>(
    lines: &'a Vec<String>,
    get_tokens_from_line: TExtractor,
    frequencies: HashMap<T, u32>,
) -> CompressedData<T>
where
    T: Hash + Eq + Clone + Display + Send + Sync,
    TExtractor: Fn(&'a str) -> TokenIterator + Send + Sync,
    TokenIterator: Iterator<Item = T>,
{
    let huffman_tree = HuffmanTree::from_frequencies(&frequencies);
    let encoder = HuffmanEncoder::from_huffman_tree(huffman_tree);
    let decoder = encoder.decoder.clone();

    let data = lines
        .par_iter()
        .map(|line| {
            get_tokens_from_line(line)
                .map(|token| encoder.encoder.get(&token).unwrap().clone())
                .fold(BitVec::new(), |mut vec1, vec2| {
                    vec1.extend(vec2);
                    vec1
                })
        })
        .collect();
    CompressedData { decoder, data }
}

#[derive(Serialize, Deserialize)]
pub struct CompressedData<T> {
    pub data: Vec<BitVec>,
    pub decoder: HashMap<BitVec, T>,
}
