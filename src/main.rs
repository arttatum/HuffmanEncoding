use rmp_serde;
use std::fs;

use compressor::application::input::Summary;
use compressor::encoding::huffman;
use compressor::encoding::huffman::{CompressedData, HuffmanEncoder};

pub fn main() {
    // Encode
    let summary = Summary::chars_from_reader(std::io::stdin().lock());
    let compressed = huffman::compress(summary.input.chars().into_iter(), summary.frequencies);

    let compressed_file_path = "/tmp/compressed_huff.mv";
    rmp_serde::encode::write(
        &mut fs::File::create(compressed_file_path).unwrap(),
        &compressed,
    )
    .unwrap();

    println!("Wrote compressed file to: {}", compressed_file_path);

    // Decode
    let deserialized_data: CompressedData<char> =
        rmp_serde::decode::from_read(fs::File::open("/tmp/compressed_huff.mv").unwrap()).unwrap();

    let decoded_text = HuffmanEncoder::decode(deserialized_data.decoder, deserialized_data.data);
    println!(
        "Compression ratio: {}%",
        compressed.data.len() / 8 * 100 / decoded_text.len()
    );

    let decompressed_file_path = "/tmp/huffman_decoded.txt";

    fs::write(decompressed_file_path, decoded_text).unwrap();

    println!("Wrote decompressed file to: {}", decompressed_file_path);
}
