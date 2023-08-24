use rmp_serde;
use std::fs;

use clap::Parser;
use compressor::{
    application::parser::TokenParser,
    encoding::huffman::{self, CompressedData, HuffmanEncoder},
};

#[derive(clap::ValueEnum, Clone, Debug)]
enum Mode {
    Compress,
    Decompress,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum TokenType {
    Chars,
    Words,
}
/// A tool for compression and decompression
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = Mode::Compress)]
    #[clap(value_enum)]
    mode: Mode,

    #[arg(short, long, default_value_t = TokenType::Chars)]
    #[clap(value_enum)]
    token_type: TokenType,
}

pub fn main() {
    let cli = Args::parse();

    // You can check the value provided by positional arguments, or option arguments
    match cli.mode {
        Mode::Compress => {
            match cli.token_type {
                TokenType::Chars => {
                    let input_data = TokenParser::chars_from_reader(std::io::stdin().lock());
                    let compressed = huffman::compress(
                        &input_data.lines,
                        |line| line.chars(),
                        input_data.token_frequencies,
                    );
                    let compressed_file_path = "/tmp/compressed_huff.mv";
                    let data: Vec<u8> = rmp_serde::encode::to_vec(&compressed).unwrap();
                    fs::write(compressed_file_path, data).unwrap();
                    println!("Wrote compressed file to: {}", compressed_file_path);
                }
                TokenType::Words => {
                    let input_data = TokenParser::strs_from_reader(std::io::stdin().lock());
                    let compressed = huffman::compress(
                        &input_data.lines,
                        |line| line.split_inclusive(" ").map(|token| token.to_string()),
                        input_data.token_frequencies,
                    );
                    let compressed_file_path = "/tmp/compressed_huff.mv";
                    rmp_serde::encode::write(
                        &mut fs::File::create(compressed_file_path).unwrap(),
                        &compressed,
                    )
                    .unwrap();
                    println!("Wrote compressed file to: {}", compressed_file_path);
                }
            };
        }
        Mode::Decompress => {
            // Decode
            let deserialized_data: CompressedData<char> =
                rmp_serde::decode::from_read(fs::File::open("/tmp/compressed_huff.mv").unwrap())
                    .unwrap();

            let decoded_text = HuffmanEncoder::decode(
                deserialized_data.decoder,
                &deserialized_data.data,
                |tokens: Vec<char>| tokens.into_iter().collect(),
            );
            //println!(
            //  "Compression ratio: {}%",
            //compressed.data.len() / 8 * 100 / decoded_text.len()
            //);

            let decompressed_file_path = "/tmp/huffman_decoded.txt";

            fs::write(decompressed_file_path, decoded_text).unwrap();

            println!("Wrote decompressed file to: {}", decompressed_file_path);
        }
    }
}
