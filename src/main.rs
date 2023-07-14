use std::fs;

fn main() {
    let provider = input::TextProvider::from_stdin();
    let huffman_tree = huffman::Tree::from_frequencies(provider.get_frequencies());
    let encoder = huffman::Encoder::from_huffman_tree(huffman_tree);

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

mod input {
    use std::collections::HashMap;
    use std::io::BufRead;
    pub struct TextProvider<TokenType> {
        text: String,
        frequencies: HashMap<TokenType, u32>,
    }

    impl TextProvider<char> {
        pub fn from_stdin() -> Self {
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

        pub fn get_text(&self) -> &str {
            &self.text
        }

        pub fn get_frequencies(&self) -> &HashMap<char, u32> {
            &self.frequencies
        }
    }
}

mod huffman {
    use bitvec::prelude::*;
    use std::borrow::BorrowMut;
    use std::collections::{BinaryHeap, HashMap};
    use std::hash::Hash;

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
        TokenType: Copy,
    {
        pub fn from_huffman_tree(tree: Box<Tree<TokenType>>) -> Self {
            println!("Generating encoder and decoder from tree");
            let mut encoder = HashMap::new();
            let mut decoder = HashMap::new();
            Encoder::get_encoding_from_node(
                tree,
                &mut BitVec::new(),
                encoder.borrow_mut(),
                decoder.borrow_mut(),
            );
            Encoder { encoder, decoder }
        }

        fn get_encoding_from_node(
            current_node: Box<Tree<TokenType>>,
            encoding: &mut BitVec,
            encoder: &mut HashMap<TokenType, BitVec>,
            decoder: &mut HashMap<BitVec, TokenType>,
        ) {
            if let Some(token) = current_node.token {
                encoder.insert(token, encoding.clone());
                decoder.insert(encoding.clone(), token);
            } else {
                if let Some(left_child) = current_node.left {
                    encoding.push(false);
                    Encoder::get_encoding_from_node(left_child, encoding, encoder, decoder);
                    encoding.pop();
                }

                if let Some(right_child) = current_node.right {
                    encoding.push(true);
                    Encoder::get_encoding_from_node(right_child, encoding, encoder, decoder);
                    encoding.pop();
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

    pub struct Tree<TokenType> {
        token: Option<TokenType>,
        count: u32,
        left: Option<Box<Tree<TokenType>>>,
        right: Option<Box<Tree<TokenType>>>,
    }

    impl<TokenType: Copy> Tree<TokenType> {
        pub fn from_frequencies(counts: &HashMap<TokenType, u32>) -> Box<Tree<TokenType>> {
            println!("Building Huffman Tree from frequency map");
            let mut heap = BinaryHeap::new();
            for (key, value) in counts.iter() {
                heap.push(Box::new(Tree {
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

    fn merge_nodes<TokenType: Copy>(
        smaller_node: Box<Tree<TokenType>>,
        larger_node: Box<Tree<TokenType>>,
    ) -> Box<Tree<TokenType>> {
        Box::new(Tree {
            token: None,
            count: smaller_node.count + larger_node.count,
            left: Some(smaller_node),
            right: Some(larger_node),
        })
    }

    impl<TokenType: Copy> Eq for Tree<TokenType> {}

    impl<TokenType: Copy> PartialEq for Tree<TokenType> {
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
    impl<TokenType: Copy> PartialOrd for Tree<TokenType> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            other.count.partial_cmp(&self.count)
        }
    }

    impl<TokenType: Copy> Ord for Tree<TokenType> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.partial_cmp(self).unwrap()
        }
    }
}
