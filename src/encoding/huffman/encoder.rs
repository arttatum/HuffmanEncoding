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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tree() -> Box<HuffmanTree<char>> {
        let counts = HashMap::from([('a', 10), ('!', 38), ('😆', 12)]);
        HuffmanTree::from_frequencies(&counts)
    }

    #[test]
    fn test_from_huffman_tree() {
        let encoder = HuffmanEncoder::from_huffman_tree(create_tree());

        let expected_bits_for_a = bitvec![0, 0];
        let expected_bits_for_exclaim = bitvec![1];
        let expected_bits_for_lols = bitvec![0, 1];

        let bits_for_a = encoder.encoder.get(&'a').unwrap();
        assert_eq!(bits_for_a.len(), 2);
        assert_eq!(bits_for_a[0], expected_bits_for_a[0]);
        assert_eq!(bits_for_a[1], expected_bits_for_a[1]);
        assert_eq!(encoder.decoder.get(bits_for_a).unwrap(), &'a');

        let bits_for_exclaim = encoder.encoder.get(&'!').unwrap();
        assert_eq!(bits_for_exclaim.len(), 1);
        assert_eq!(bits_for_exclaim[0], expected_bits_for_exclaim[0]);
        assert_eq!(encoder.decoder.get(bits_for_exclaim).unwrap(), &'!');

        let bits_for_lols = encoder.encoder.get(&'😆').unwrap();
        assert_eq!(bits_for_lols.len(), 2);
        assert_eq!(bits_for_lols[0], expected_bits_for_lols[0]);
        assert_eq!(bits_for_lols[1], expected_bits_for_lols[1]);
        assert_eq!(encoder.decoder.get(bits_for_lols).unwrap(), &'😆');
    }

    #[test]
    fn test_encode_decode_returns_original_input() {
        let encoder = HuffmanEncoder::from_huffman_tree(create_tree());
        let input = "!!!!!!!!!!😆!😆!a!😆!a!😆!a!😆!a!😆!a!😆!a!😆!a!😆!a!😆!a!😆!a!😆!!!!!!!!!!";
        assert_eq!(input, encoder.decode(&encoder.encode(input)));
    }
}
