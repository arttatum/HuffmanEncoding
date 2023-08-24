use rmp_serde;
use std::{
    fs,
    io::{self, BufReader, Write},
};

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

#[derive(clap::ValueEnum, Clone, Debug)]
enum Source {
    Stdin,
    File,
}

/// A compression and decompression tool.
///
/// The default behaviour is to compress stdin to stdout. Optionally, input and output file paths may be provided.
///
/// During compression, the text is broken into 'tokens', either chars or words. Depending on the workload, compression ratio and speed may be better for one choice or the other. The default token type is 'chars'.
///
/// To decompress a file: compressor --mode=decompress --file=$PATH_TO_COMPRESSED_FILE
/// --token-type=$TOKEN_TYPE_USED_DURING_COMPRESSION
///
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    #[arg(short, long, default_value_t = Mode::Compress)]
    #[clap(value_enum)]
    mode: Mode,

    #[arg(short, long, default_value_t = TokenType::Chars)]
    #[clap(value_enum)]
    token_type: TokenType,

    /// File path of input, otherwise the compressor reads from stdin.
    #[arg(short, long)]
    in_file: Option<String>,

    /// File path of output, otherwise the compressor writes to stdout.
    #[arg(short, long)]
    out_file: Option<String>,
}

pub fn main() {
    let cli = Args::parse();

    let data = match cli.mode {
        Mode::Compress => {
            println!("Compressing text...");
            match cli.token_type {
                TokenType::Chars => {
                    println!("Generating char tokens...");
                    let input_data = match cli.in_file {
                        None => TokenParser::chars_from_reader(std::io::stdin().lock()),
                        Some(s) => TokenParser::chars_from_reader(BufReader::new(
                            fs::File::open(s).unwrap(),
                        )),
                    };
                    println!("Performing Huffman Compression...");
                    let compressed = huffman::compress(
                        &input_data.lines,
                        |line| line.chars(),
                        input_data.token_frequencies,
                    );
                    println!("Encoding into MessagePack format...");
                    rmp_serde::encode::to_vec(&compressed).unwrap()
                }
                TokenType::Words => {
                    println!("Generating word tokens...");
                    let input_data = match cli.in_file {
                        None => TokenParser::strs_from_reader(std::io::stdin().lock()),
                        Some(s) => TokenParser::strs_from_reader(BufReader::new(
                            fs::File::open(s).unwrap(),
                        )),
                    };

                    println!("Performing Huffman Compression...");
                    let compressed = huffman::compress(
                        &input_data.lines,
                        |line| line.split_inclusive(" ").map(|token| token.to_string()),
                        input_data.token_frequencies,
                    );
                    println!("Encoding into MessagePack...");
                    rmp_serde::encode::to_vec(&compressed).unwrap()
                }
            }
        }
        Mode::Decompress => {
            let input_file_path = match cli.in_file {
                Some(s) => s,
                None => panic!("Please provide the filepath of the file to decompress."),
            };
            println!("Decompressing text...");
            match cli.token_type {
                TokenType::Chars => {
                    println!("Deserializing from MessagePack...");
                    let deserialized_data: CompressedData<char> =
                        rmp_serde::decode::from_read(fs::File::open(input_file_path).unwrap())
                            .unwrap();
                    println!("Decoding text...");
                    HuffmanEncoder::decode(
                        deserialized_data.decoder,
                        &deserialized_data.data,
                        |tokens: Vec<char>| tokens.into_iter().collect(),
                    )
                }
                TokenType::Words => {
                    println!("Deserializing from MessagePack...");
                    let deserialized_data: CompressedData<String> =
                        rmp_serde::decode::from_read(fs::File::open(input_file_path).unwrap())
                            .unwrap();

                    println!("Decoding text...");
                    HuffmanEncoder::decode(
                        deserialized_data.decoder,
                        &deserialized_data.data,
                        |tokens: Vec<String>| tokens.join(""),
                    )
                }
            }
        }
    };
    match cli.out_file {
        Some(s) => {
            println!("Writing to file: {s}");
            fs::write(s, data).unwrap();
        }
        None => {
            io::stdout().write(&data).unwrap();
        }
    }

    println!("Done!")
}
