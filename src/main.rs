use std::fs;
use std::fmt::Display;
mod encoding;
mod application;

use encoding::huffman::{Tree, Encoder};
use application::input::TextProvider;

fn main() {
    let provider = TextProvider::from_stdin();
    let huffman_tree = Tree::from_frequencies(provider.get_frequencies());

    let encoder = Encoder::from_huffman_tree(huffman_tree);

    let encoded_string = encoder.encode(provider.get_text());
    let decoded_string = encoder.decode(&encoded_string);

    let encoded_text_size = encoded_string.len() / 8 + {
        if encoded_string.len() % 8 == 0 {
            0
        } else {
            1
        }
    };
    println!("Original text consumes {} bytes", provider.get_text().len());
    println!("Encoded text consumes {} bytes", encoded_text_size);

    println!("Decoded text consumes {} bytes", decoded_string.len());

    println!(
        "Compression ratio: {}%",
        encoded_text_size * 100 / decoded_string.len()
    );

    fs::write("/tmp/huffmanEncoded.txt", decoded_string).unwrap();
}


fn walk<T: Copy + Display>(tree: Box<Tree<T>>) { 
    match *tree {
        Tree::Leaf(leaf) => println!("Found a leaf, representing {} {}'s", leaf.count, leaf.token),
        Tree::InternalNode(node) => {
            println!("Walking left");
            walk(node.left);
            println!("Walking right");
            walk(node.right);
        }
    }
}
