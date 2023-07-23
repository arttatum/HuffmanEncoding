use rmp_serde;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

mod application;
mod encoding;

use application::input::Input;
use bitvec::vec::BitVec;
use encoding::{HuffmanEncoder, HuffmanTree};

fn main() {
    let mut input = Input::from_source(std::io::stdin().lock());

    let summary = input.process_as_chars();

    let huffman_tree = HuffmanTree::from_frequencies(&summary.frequencies);

    let encoder = HuffmanEncoder::from_huffman_tree(huffman_tree);

    let encoded_text = encoder.encode(&summary.text);

    let encoded_text_size = encoded_text.len() / 8 + {
        if encoded_text.len() % 8 == 0 {
            0
        } else {
            1
        }
    };
    println!("Original text consumes {} bytes", &summary.text.len());
    println!("Encoded text consumes {} bytes", encoded_text_size);

    let data = CompressedData {
        decoder: encoder.decoder,
        data: encoded_text,
    };

    rmp_serde::encode::write(
        &mut fs::File::create("/tmp/compressed_huff.mv").unwrap(),
        &data,
    )
    .unwrap();

    // Decode
    let deserialized_data: CompressedData<char> =
        rmp_serde::decode::from_read(fs::File::open("/tmp/compressed_huff.mv").unwrap()).unwrap();

    let decoded_text = HuffmanEncoder::decode(deserialized_data.decoder, &deserialized_data.data);
    println!("Decoded text consumes {} bytes", decoded_text.len());

    println!(
        "Compression ratio: {}%",
        encoded_text_size * 100 / decoded_text.len()
    );

    fs::write("/tmp/huffman_decoded.txt", decoded_text).unwrap();
}

#[derive(Serialize, Deserialize)]
struct CompressedData<T> {
    decoder: HashMap<BitVec, T>,
    data: BitVec,
}
