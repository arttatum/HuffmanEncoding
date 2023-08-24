use crate::encoding::huffman::tree::HuffmanTree;
use bit_vec::BitVec;
use rayon::prelude::*;
use std::{collections::HashMap, hash::Hash};

#[derive(Clone)]
pub struct HuffmanEncoder<T>
where
    T: Hash + Eq + Sync,
{
    pub encoder: HashMap<T, BitVec>,
    pub decoder: HashMap<BitVec, T>,
}

impl<T> HuffmanEncoder<T>
where
    T: Hash + Eq + Clone + Send + Sync,
{
    pub fn from_huffman_tree(tree: Box<HuffmanTree<T>>) -> Self {
        let mut encoder = HashMap::new();
        let mut decoder = HashMap::new();
        HuffmanEncoder::get_encoding_from_node(tree, BitVec::new(), &mut encoder, &mut decoder);
        HuffmanEncoder { encoder, decoder }
    }

    fn get_encoding_from_node<'a>(
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

    pub fn encode<'a, TokenIterator>(
        self,
        input: &'a Vec<String>,
        get_tokens_from_line: impl Fn(&'a str) -> TokenIterator + Send + Sync,
    ) -> Vec<BitVec>
    where
        TokenIterator: Iterator<Item = T>,
    {
        input
            .par_iter()
            .map(|line| {
                get_tokens_from_line(line)
                    .map(|token| self.encoder.get(&token).unwrap().clone())
                    .fold(BitVec::new(), |mut vec1, vec2| {
                        vec1.extend(vec2);
                        vec1
                    })
            })
            .collect()
    }

    pub fn decode(
        decoder: HashMap<BitVec, T>,
        input: &Vec<BitVec>,
        tokens_to_line: impl Fn(Vec<T>) -> String + Send + Sync,
    ) -> Vec<u8> {
        input
            .par_iter()
            .map(|bits| {
                let mut candidate = BitVec::new();
                let mut tokens = Vec::new();
                for bit in bits {
                    candidate.push(bit);
                    if let Some(entry) = decoder.get(&candidate) {
                        tokens.push((*entry).clone());
                        candidate = BitVec::new();
                    }
                }
                tokens_to_line(tokens)
            })
            .collect::<String>()
            .into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_huffman_tree() {
        let counts = HashMap::from([('a', 10), ('!', 38), ('😆', 12)]);
        let tree = HuffmanTree::from_frequencies(&counts);
        let encoder = HuffmanEncoder::from_huffman_tree(tree);

        let expected_bits_for_a = vec![false, false];
        let expected_bits_for_exclaim = vec![true];
        let expected_bits_for_lols = vec![false, true];

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
        let counts = HashMap::from([
            ('h', 1),
            ('i', 1),
            ('1', 1),
            ('2', 1),
            ('A', 1),
            ('|', 1),
            ('Z', 1),
            ('a', 2),
            ('!', 4),
            ('😆', 1),
            ('\n', 1),
        ]);
        let tree = HuffmanTree::from_frequencies(&counts);
        let encoder = HuffmanEncoder::from_huffman_tree(tree);
        let input = "!!hi!\na!😆\n12aA|Z";
        let input_lines: Vec<String> = input
            .split_inclusive("\n")
            .map(|s| String::from(s))
            .collect();
        let encoded_text = encoder.clone().encode(&input_lines, |line| line.chars());
        assert_eq!(
            input.as_bytes(),
            HuffmanEncoder::decode(
                encoder.decoder.clone(),
                &encoded_text,
                |tokens: Vec<char>| tokens.into_iter().collect::<String>()
            )
        );
    }
}
