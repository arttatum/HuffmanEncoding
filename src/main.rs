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
        Mode::Compress => compress(cli.token_type, cli.in_file)?,
        Mode::Decompress => decompress(cli.token_type, cli.in_file)?,
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

fn compress(
    token_type: TokenType,
    input_file: Option<String>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    info!("Compressing text...");
    match token_type {
        TokenType::Chars => {
            info!("Generating char tokens...");
            let input_data = match input_file {
                Some(s) => TokenParser::chars_from_reader(BufReader::new(fs::File::open(s)?)),
                None => TokenParser::chars_from_reader(std::io::stdin().lock()),
            };

            info!("Performing Huffman Compression...");
            let compressed =
                huffman::compress(&input_data.lines, input_data.token_frequencies, |line| {
                    line.chars()
                });

            info!("Encoding into MessagePack format...");
            Ok(rmp_serde::encode::to_vec(&compressed)?)
        }
        TokenType::Words => {
            info!("Generating word tokens...");
            let input_data = match input_file {
                Some(s) => TokenParser::words_from_reader(BufReader::new(fs::File::open(s)?)),
                None => TokenParser::words_from_reader(std::io::stdin().lock()),
            };

            info!("Performing Huffman Compression...");
            let compressed =
                huffman::compress(&input_data.lines, input_data.token_frequencies, |line| {
                    line.split_inclusive(' ').map(|token| token.to_string())
                });

            info!("Encoding into MessagePack...");
            Ok(rmp_serde::encode::to_vec(&compressed)?)
        }
    }
}

fn decompress(
    token_type: TokenType,
    input_file: Option<String>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    info!("Decompressing text...");
    match token_type {
        TokenType::Chars => {
            info!("Deserializing from MessagePack...");
            let deserialized_data: CompressedData<char> = match input_file {
                Some(s) => rmp_serde::decode::from_read(fs::File::open(s)?)?,
                None => rmp_serde::decode::from_read(std::io::stdin().lock())?,
            };

            info!("Decoding text...");
            Ok(HuffmanEncoder::decode(
                deserialized_data.decoder,
                &deserialized_data.data,
                |tokens: Vec<char>| tokens.into_iter().collect(),
            ))
        }
        TokenType::Words => {
            info!("Deserializing from MessagePack...");
            let deserialized_data: CompressedData<String> = match input_file {
                Some(s) => rmp_serde::decode::from_read(fs::File::open(s)?)?,
                None => rmp_serde::decode::from_read(std::io::stdin().lock())?,
            };

            info!("Decoding text...");
            Ok(HuffmanEncoder::decode(
                deserialized_data.decoder,
                &deserialized_data.data,
                |tokens: Vec<String>| tokens.join(""),
            ))
        }
    }
}
