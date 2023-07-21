use std::fs;

mod application;
mod encoding;

use application::input::Input;
use encoding::huffman::{Encoder, Tree};

fn main() {
    let mut input = Input::from_source(std::io::stdin().lock());

    let summary = input.process_as_chars();

    let huffman_tree = Tree::from_frequencies(&summary.frequencies);

    let encoder = Encoder::from_huffman_tree(huffman_tree);

    let encoded_text = encoder.encode(&summary.text);

    let decoded_text = encoder.decode(&encoded_text);

    let encoded_text_size = encoded_text.len() / 8 + {
        if encoded_text.len() % 8 == 0 {
            0
        } else {
            1
        }
    };
    println!("Original text consumes {} bytes", &summary.text.len());
    println!("Encoded text consumes {} bytes", encoded_text_size);

    println!("Decoded text consumes {} bytes", decoded_text.len());

    println!(
        "Compression ratio: {}%",
        encoded_text_size * 100 / decoded_text.len()
    );

    fs::write("/tmp/huffman_decoded.txt", decoded_text).unwrap();
}
