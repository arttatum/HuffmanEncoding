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
    pub encoder: HashMap<T, BitVec>,
    pub decoder: HashMap<BitVec, T>,
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
        let mut encoder = HashMap::new();
        let mut decoder = HashMap::new();
        HuffmanEncoder::get_encoding_from_node(
            tree,
            BitVec::new(),
            encoder.borrow_mut(),
            decoder.borrow_mut(),
        );
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

    pub fn encode<I>(self, input_iter: I) -> BitVec
    where
        I: Iterator<Item = T>,
    {
        let mut output = BitVec::new();
        for b in input_iter {
            if let Some(encoding) = self.encoder.get(&b) {
                for bb in encoding {
                    output.push(*bb);
                }
            }
        }
        return output;
    }
}

impl HuffmanEncoder<char> {
    pub fn decode(decoder: HashMap<BitVec, char>, input: BitVec) -> String {
        let mut output = String::new();
        let mut candidate = BitVec::new();
        for b in input {
            candidate.push(b);
            if let Some(entry) = decoder.get(&candidate) {
                output.push(char::try_from(*entry).unwrap());
                candidate = BitVec::new();
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
        assert_eq!(
            input,
            HuffmanEncoder::decode(
                encoder.decoder.clone(),
                encoder.encode(input.chars().into_iter())
            )
        );
    }
}
