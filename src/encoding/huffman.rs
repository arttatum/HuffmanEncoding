use bitvec::prelude::*;
use core::fmt::Display;
use std::borrow::BorrowMut;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

#[derive(Clone, PartialEq, Eq)]
pub enum Tree<T: Clone> {
    Leaf {
        token: T,
        count: u32,
    },
    InternalNode {
        count: u32,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}

impl<T: Clone + Eq> Ord for Tree<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.get_count().cmp(&self.get_count())
    }
}

impl<T: Clone + Eq> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Clone + Eq> Tree<T> {
    pub fn from_frequencies(counts: &HashMap<T, u32>) -> Box<Tree<T>> {
        println!("Building Huffman Tree using token frequency map");
        let mut heap = BinaryHeap::new();
        for (key, value) in counts.iter() {
            heap.push(Box::new(Tree::Leaf {
                token: key.clone(),
                count: value.clone(),
            }));
        }

        while heap.len() > 1 {
            let smaller_node = heap.pop().unwrap();
            let larger_node = heap.pop().unwrap();
            let parent_node = Box::new(Tree::InternalNode {
                count: (*smaller_node).get_count() + (*larger_node).get_count(),
                left: smaller_node,
                right: larger_node,
            });
            heap.push(parent_node);
        }

        heap.pop().unwrap()
    }

    fn get_count(&self) -> u32 {
        match self {
            Tree::Leaf { count, .. } => *count,
            Tree::InternalNode { count, .. } => *count,
        }
    }
}

#[derive(Clone)]
pub struct Encoder<T>
where
    T: Hash,
    T: Eq,
    T: PartialEq,
{
    encoder: HashMap<T, BitVec>,
    decoder: HashMap<BitVec, T>,
}

impl<T> Encoder<T>
where
    T: Hash,
    T: Eq,
    T: PartialEq,
    T: Clone,
    T: Display,
{
    pub fn from_huffman_tree(tree: Box<Tree<T>>) -> Self {
        println!("Generating encoder and decoder from tree");
        let mut encoder = HashMap::new();
        let mut decoder = HashMap::new();
        Encoder::get_encoding_from_node(
            tree,
            BitVec::new(),
            encoder.borrow_mut(),
            decoder.borrow_mut(),
        );
        let mut sorted_encoder: Vec<_> = encoder.iter().collect();
        sorted_encoder.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        for (c, f) in sorted_encoder.iter() {
            println!("Encoder: {}: {}", c, f);
        }
        Encoder { encoder, decoder }
    }

    fn get_encoding_from_node(
        current_node: Box<Tree<T>>,
        encoding: BitVec,
        encoder: &mut HashMap<T, BitVec>,
        decoder: &mut HashMap<BitVec, T>,
    ) {
        match *current_node {
            Tree::Leaf { token, .. } => {
                encoder.insert(token.clone(), encoding.clone());
                decoder.insert(encoding.clone(), token.clone());
            }
            Tree::InternalNode { left, right, .. } => {
                let mut left_encoding = encoding.clone();
                left_encoding.push(false);
                Encoder::get_encoding_from_node(left, left_encoding, encoder, decoder);

                let mut right_encoding = encoding.clone();
                right_encoding.push(true);
                Encoder::get_encoding_from_node(right, right_encoding, encoder, decoder);
            }
        }
    }
}

impl Encoder<char> {
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
