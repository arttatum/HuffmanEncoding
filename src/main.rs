use bitvec::prelude::*;
use std::borrow::BorrowMut;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, Read};

fn main() {
    println!("Computing frequency of bytes...");
    let mut counts = HashMap::new();
    let mut line = String::new();
    let mut input = String::new();
    while let Ok(n_bytes) = io::stdin().lock().read_to_string(&mut line) {
        if n_bytes == 0 {
            break;
        }
        for c in line.as_bytes() {
            if let Some(value) = counts.get_mut(c) {
                *value += 1;
            } else {
                counts.insert(*c, 1);
            }
        }
        input.push_str(&line);
        line.clear();
    }

    let mut heap = BinaryHeap::new();

    for (key, value) in counts.iter() {
        heap.push(Box::new(HuffmanTreeNode {
            letter: Some(*key),
            count: value.clone(),
            left: None,
            right: None,
        }));
    }

    println!("Building a Huffman Tree to {{en,de}}code the given text...");
    println!("The algorithm to build the Huffman Tree is as follows:");
    println!("\t1) From the MinHeap, pop the nodes corresponding to the least frequently occurring letters.");
    println!("\t2) Create a new node, with a value equal to the sum of the two child nodes.");
    println!("\t3) Reference the child nodes from the new node, with a consistent ordering (e.g. left = smaller).");
    println!("\t4) Push the new node onto the MinHeap.");
    println!("\t5) Repeat 1-4, until there is only one node left in the MinHeap.");
    println!("The remaining node in the heap is the root of the HuffmanTree.");

    while heap.len() > 1 {
        let smaller_node = heap.pop().unwrap();
        let larger_node = heap.pop().unwrap();
        heap.push(merge_nodes(smaller_node, larger_node));
    }

    let mut encodings = HashMap::new();
    let mut decoding_map = HashMap::new();

    // Traverse tree to form mapping from bytes to encodings
    get_coding_from_tree(heap.pop().unwrap(), &mut BitVec::new(), encodings.borrow_mut(), decoding_map.borrow_mut());

    
    println!("Given these encodings, we can encode the original text");


    let encoded_string = encode_string(&mut encodings, &input);
    println!("{}", encoded_string);

    let decoded_string = decode_string(&encoded_string, &decoding_map);

    println!("{}", decoded_string);
}

fn decode_string(input: &BitVec, decoding_map: &HashMap<BitVec, u8>) -> String {
    let mut output = String::new();
    let mut candidate = BitVec::new();
    for b in input {
        candidate.push(*b);
        if let Some(entry) = decoding_map.get(&candidate) {
            output.push(char::try_from(*entry).unwrap());
            candidate = BitVec::new();
        } 
    }
    return output;
}

fn encode_string(encodings: &mut HashMap<u8, BitVec>, input: &str) -> BitVec {
    let mut output = BitVec::new();
    for b in input.as_bytes().iter() {
        let encoding = encodings.entry(*b).or_default();
        for bb in encoding {
            output.push(*bb);
        }
    }
    return output;
}

fn get_coding_from_tree(
    tree: Box<HuffmanTreeNode>,
    encoding: &mut BitVec,
    encodings: &mut HashMap<u8, BitVec>,
    decoding_map: &mut HashMap<BitVec, u8>,
) {
    if let Some(letter) = tree.letter {
        encodings.insert(letter, encoding.clone());
        decoding_map.insert(encoding.clone(), letter);
    } else {
        if let Some(left_child) = tree.left {
            encoding.push(false);
            get_coding_from_tree(left_child, encoding, encodings, decoding_map);
            encoding.pop();
        }

        if let Some(right_child) = tree.right {
            encoding.push(true);
            get_coding_from_tree(right_child, encoding, encodings, decoding_map);
            encoding.pop();
        }
    }
}

struct HuffmanTreeNode {
    letter: Option<u8>,
    count: u32,
    left: Option<Box<HuffmanTreeNode>>,
    right: Option<Box<HuffmanTreeNode>>,
}

fn merge_nodes(
    smaller_node: Box<HuffmanTreeNode>,
    larger_node: Box<HuffmanTreeNode>,
) -> Box<HuffmanTreeNode> {
    Box::new(HuffmanTreeNode {
        letter: None,
        count: smaller_node.count + larger_node.count,
        left: Some(smaller_node),
        right: Some(larger_node),
    })
}

impl Eq for HuffmanTreeNode {}

impl PartialEq for HuffmanTreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

// Order is determined such that one node is considered 'larger' than the other if it has
// a lower count. This is to allow usage in a MaxHeap (BinaryHeap) that we need to behave like a
// MinHeap.
//
// An alternative approach would be to use Reverse on each node, prior to insertion into the
// MaxHeap.
impl PartialOrd for HuffmanTreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.count.partial_cmp(&self.count)
    }
}

impl Ord for HuffmanTreeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.partial_cmp(self).unwrap()
    }
}
