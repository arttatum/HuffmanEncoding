use crate::encoding::huffman::tree::HuffmanTree;
use bitvec::prelude::*;
use core::fmt::Display;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
pub struct HuffmanEncoder<T>
where
    T: Hash,
    T: Eq,
    T: PartialEq,
{
    encoder: HashMap<T, BitVec>,
    decoder: HashMap<BitVec, T>,
}

impl<T> HuffmanEncoder<T>
where
    T: Hash,
    T: Eq,
    T: PartialEq,
    T: Clone,
    T: Display,
{
    pub fn from_huffman_tree(tree: Box<HuffmanTree<T>>) -> Self {
        println!("Generating encoder and decoder from tree");
        let mut encoder = HashMap::new();
        let mut decoder = HashMap::new();
        HuffmanEncoder::get_encoding_from_node(
            tree,
            BitVec::new(),
            encoder.borrow_mut(),
            decoder.borrow_mut(),
        );
        let mut sorted_encoder: Vec<_> = encoder.iter().collect();
        sorted_encoder.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        for (c, f) in sorted_encoder.iter() {
            println!("HuffmanEncoder: {}: {}", c, f);
        }
        HuffmanEncoder { encoder, decoder }
    }

    fn get_encoding_from_node(
        current_node: Box<HuffmanTree<T>>,
        encoding: BitVec,
        encoder: &mut HashMap<T, BitVec>,
        decoder: &mut HashMap<BitVec, T>,
    ) {
        match *current_node {
            HuffmanTree::Leaf { token, .. } => {
                encoder.insert(token.clone(), encoding.clone());
                decoder.insert(encoding.clone(), token.clone());
            }
            HuffmanTree::InternalNode { left, right, .. } => {
                let mut left_encoding = encoding.clone();
                left_encoding.push(false);
                HuffmanEncoder::get_encoding_from_node(left, left_encoding, encoder, decoder);

                let mut right_encoding = encoding.clone();
                right_encoding.push(true);
                HuffmanEncoder::get_encoding_from_node(right, right_encoding, encoder, decoder);
            }
        }
    }
}

impl HuffmanEncoder<char> {
    pub fn decode(&self, input: &BitVec) -> String {
        println!("Decoding text");
        let mut output = String::new();
        let mut candidate = BitVec::new();
        for b in input {
            candidate.push(*b);
            if let Some(entry) = self.decoder.get(&candidate) {
                output.push(char::try_from(*entry).unwrap());
                candidate = BitVec::new();
            }
        }
        return output;
    }

    pub fn encode(&self, input: &str) -> BitVec {
        println!("Encoding text");
        let mut output = BitVec::new();
        for b in input.chars().into_iter() {
            if let Some(encoding) = self.encoder.get(&b) {
                for bb in encoding {
                    output.push(*bb);
                }
            }
        }
        return output;
    }
}
