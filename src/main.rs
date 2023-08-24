use rmp_serde;
use std::{
    fs,
    io::{self, BufReader, Write},
};
#[macro_use]
extern crate log;

use compressor::{
    application::{
        cli::{Args, Mode, Parser, TokenType},
        parser::TokenParser,
    },
    encoding::huffman::{self, CompressedData, HuffmanEncoder},
};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Args::parse();
    env_logger::init();

    let data = match cli.mode {
        Mode::Compress => {
            info!("Compressing text...");
            match cli.token_type {
                TokenType::Chars => {
                    info!("Generating char tokens...");
                    let input_data = match cli.in_file {
                        None => TokenParser::chars_from_reader(std::io::stdin().lock()),
                        Some(s) => {
                            TokenParser::chars_from_reader(BufReader::new(fs::File::open(s)?))
                        }
                    };

                    info!("Performing Huffman Compression...");
                    let compressed = huffman::compress(
                        &input_data.lines,
                        |line| line.chars(),
                        input_data.token_frequencies,
                    );

                    info!("Encoding into MessagePack format...");
                    rmp_serde::encode::to_vec(&compressed)?
                }
                TokenType::Words => {
                    info!("Generating word tokens...");
                    let input_data = match cli.in_file {
                        None => TokenParser::strs_from_reader(std::io::stdin().lock()),
                        Some(s) => {
                            TokenParser::strs_from_reader(BufReader::new(fs::File::open(s)?))
                        }
                    };

                    info!("Performing Huffman Compression...");
                    let compressed = huffman::compress(
                        &input_data.lines,
                        |line| line.split_inclusive(" ").map(|token| token.to_string()),
                        input_data.token_frequencies,
                    );
                    info!("Encoding into MessagePack...");
                    rmp_serde::encode::to_vec(&compressed)?
                }
            }
        }
        Mode::Decompress => {
            info!("Decompressing text...");
            match cli.token_type {
                TokenType::Chars => {
                    info!("Deserializing from MessagePack...");

                    let deserialized_data: CompressedData<char> = match cli.in_file {
                        Some(s) => rmp_serde::decode::from_read(fs::File::open(s)?)?,
                        None => rmp_serde::decode::from_read(std::io::stdin().lock())?,
                    };

                    info!("Decoding text...");

                    HuffmanEncoder::decode(
                        deserialized_data.decoder,
                        &deserialized_data.data,
                        |tokens: Vec<char>| tokens.into_iter().collect(),
                    )
                }
                TokenType::Words => {
                    info!("Deserializing from MessagePack...");

                    let deserialized_data: CompressedData<String> = match cli.in_file {
                        Some(s) => rmp_serde::decode::from_read(fs::File::open(s)?)?,
                        None => rmp_serde::decode::from_read(std::io::stdin().lock())?,
                    };

                    info!("Decoding text...");

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
            info!("Writing to file: {s}");
            fs::write(s, data)?;
        }
        None => {
            info!("Writing to stdout.");
            io::stdout().write(&data)?;
        }
    }

    info!("Done!");
    Ok(())
}
