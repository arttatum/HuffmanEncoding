use bitvec::prelude::*;
use std::borrow::BorrowMut;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::io::BufRead;

fn main() {
    let provider = TextProvider::from_stdin();
    let huffman_tree = HuffmanTree::from_frequencies(provider.get_frequencies());
    let encoder = HuffmanEncoder::from_huffman_tree(huffman_tree);

    let encoded_string = encoder.encode(provider.get_text());
    let decoded_string = encoder.decode(&encoded_string);

    let encoded_text_size = encoded_string.len() / 8 + {
        if encoded_string.len() % 8 == 0 {
            0
        } else {
            1
        }
    };

    println!("Encoded text consumes {} bytes", encoded_text_size);

    println!("Decoded text consumes {} bytes", decoded_string.len());

    println!(
        "Compression ratio: {}%",
        encoded_text_size * 100 / decoded_string.len()
    );

    fs::write("/tmp/huffmanEncoded.txt", decoded_string).unwrap();
}

struct TextProvider<TokenType> {
    text: String,
    frequencies: HashMap<TokenType, u32>,
}

impl TextProvider<char> {
    fn from_stdin() -> Self {
        println!("Getting text and counts from stdin");
        let mut frequencies = HashMap::new();
        let mut line = String::new();
        let mut text = String::new();
        while let Ok(n_bytes) = std::io::stdin().lock().read_line(&mut line) {
            if n_bytes == 0 {
                break;
            }
            for c in line.chars().into_iter() {
                frequencies
                    .entry(c)
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
                text.push(c);
            }
            line.clear();
        }
        TextProvider { text, frequencies }
    }

    fn get_text(&self) -> &str {
        &self.text
    }

    fn get_frequencies(&self) -> &HashMap<char, u32> {
        &self.frequencies
    }
}

#[derive(Clone)]
struct HuffmanEncoder {
    encoder: HashMap<char, BitVec>,
    decoder: HashMap<BitVec, char>,
}

impl HuffmanEncoder {
    fn from_huffman_tree(tree: Box<HuffmanTree>) -> Self {
        println!("Generating encoder and decoder from tree");
        let mut encoder = HashMap::new();
        let mut decoder = HashMap::new();
        HuffmanEncoder::get_encoding_from_node(
            tree,
            &mut BitVec::new(),
            encoder.borrow_mut(),
            decoder.borrow_mut(),
        );
        HuffmanEncoder { encoder, decoder }
    }

    fn get_encoding_from_node(
        current_node: Box<HuffmanTree>,
        encoding: &mut BitVec,
        encoder: &mut HashMap<char, BitVec>,
        decoder: &mut HashMap<BitVec, char>,
    ) {
        if let Some(letter) = current_node.token {
            encoder.insert(letter, encoding.clone());
            decoder.insert(encoding.clone(), letter);
        } else {
            if let Some(left_child) = current_node.left {
                encoding.push(false);
                HuffmanEncoder::get_encoding_from_node(left_child, encoding, encoder, decoder);
                encoding.pop();
            }

            if let Some(right_child) = current_node.right {
                encoding.push(true);
                HuffmanEncoder::get_encoding_from_node(right_child, encoding, encoder, decoder);
                encoding.pop();
            }
        }
    }

    fn decode(&self, input: &BitVec) -> String {
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

    fn encode(&self, input: &str) -> BitVec {
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

struct HuffmanTree {
    token: Option<char>,
    count: u32,
    left: Option<Box<HuffmanTree>>,
    right: Option<Box<HuffmanTree>>,
}

impl HuffmanTree {
    fn from_frequencies(counts: &HashMap<char, u32>) -> Box<HuffmanTree> {
        println!("Building Huffman Tree from frequency map");
        let mut heap = BinaryHeap::new();
        for (key, value) in counts.iter() {
            heap.push(Box::new(HuffmanTree {
                token: Some(*key),
                count: value.clone(),
                left: None,
                right: None,
            }));
        }

        while heap.len() > 1 {
            let smaller_node = heap.pop().unwrap();
            let larger_node = heap.pop().unwrap();
            heap.push(merge_nodes(smaller_node, larger_node));
        }
        heap.pop().unwrap()
    }
}

fn merge_nodes(smaller_node: Box<HuffmanTree>, larger_node: Box<HuffmanTree>) -> Box<HuffmanTree> {
    Box::new(HuffmanTree {
        token: None,
        count: smaller_node.count + larger_node.count,
        left: Some(smaller_node),
        right: Some(larger_node),
    })
}

impl Eq for HuffmanTree {}

impl PartialEq for HuffmanTree {
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
impl PartialOrd for HuffmanTree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.count.partial_cmp(&self.count)
    }
}

impl Ord for HuffmanTree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.partial_cmp(self).unwrap()
    }
}
