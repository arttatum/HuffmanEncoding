use bitvec::prelude::*;
use core::fmt::Display;
use std::borrow::BorrowMut;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

#[derive(Clone, PartialEq, Eq)]
pub enum Tree<TokenType: Clone> {
    Leaf(Leaf<TokenType>),
    InternalNode(InternalNode<TokenType>),
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

#[derive(Clone, PartialEq, Eq)]
pub struct Leaf<TokenType> {
    pub token: TokenType,
    pub count: u32,
}

#[derive(Clone, PartialEq, Eq)]
pub struct InternalNode<TokenType: Clone> {
    count: u32,
    pub left: Box<Tree<TokenType>>,
    pub right: Box<Tree<TokenType>>,
}


impl<TokenType: Clone + Eq> Tree<TokenType> {
    pub fn from_frequencies(counts: &HashMap<TokenType, u32>) -> Box<Tree<TokenType>> {
        println!("Building Huffman Tree from frequency map");
        let mut heap = BinaryHeap::new();
        for (key, value) in counts.iter() {
            heap.push(Box::new(Tree::Leaf(Leaf {
                token: key.clone(),
                count: value.clone(),
            })));
        }

        while heap.len() > 1 {
            let smaller_node = heap.pop().unwrap();
            let larger_node = heap.pop().unwrap();
            heap.push(merge_nodes(smaller_node, larger_node));
        }
        heap.pop().unwrap()
    }

    fn get_count(&self) -> u32 {
        match self {
            Tree::Leaf(leaf) => leaf.count,
            Tree::InternalNode(node) => node.count,
        }
    }
}

fn merge_nodes<TokenType: Clone + Eq>(
    smaller_node: Box<Tree<TokenType>>,
    larger_node: Box<Tree<TokenType>>,
) -> Box<Tree<TokenType>> {
    Box::new(Tree::InternalNode(InternalNode {
        count: (*smaller_node).get_count() + (*larger_node).get_count(),
        left: smaller_node,
        right: larger_node,
    }))
}

#[derive(Clone)]
pub struct Encoder<TokenType>
where
    TokenType: Hash,
    TokenType: Eq,
    TokenType: PartialEq,
{
    encoder: HashMap<TokenType, BitVec>,
    decoder: HashMap<BitVec, TokenType>,
}

impl<TokenType> Encoder<TokenType>
where
    TokenType: Hash,
    TokenType: Eq,
    TokenType: PartialEq,
    TokenType: Clone,
    TokenType: Display
{
    pub fn from_huffman_tree(tree: Box<Tree<TokenType>>) -> Self {
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
        sorted_encoder.sort_by(|a, b| a.1.cmp(b.1)); 

        for (c, f) in sorted_encoder.iter() {
            println!("Encoder: {}: {}", c, f);
        }
        Encoder { encoder, decoder }
    }

    fn get_encoding_from_node(
        current_node: Box<Tree<TokenType>>,
        encoding: BitVec,
        encoder: &mut HashMap<TokenType, BitVec>,
        decoder: &mut HashMap<BitVec, TokenType>,
    ) {
        match *current_node {
            Tree::Leaf(leaf) => {
                encoder.insert(leaf.token.clone(), encoding.clone());
                decoder.insert(encoding.clone(), leaf.token.clone());
            }
            Tree::InternalNode(node) => {
                let mut left_encoding = encoding.clone();
                left_encoding.push(false);
                Encoder::get_encoding_from_node(node.left, left_encoding, encoder, decoder);

                let mut right_encoding = encoding.clone();
                right_encoding.push(true);
                Encoder::get_encoding_from_node(node.right, right_encoding, encoder, decoder);
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
