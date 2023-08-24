use compressor::application::parser::TokenParser;
use compressor::encoding::huffman::tree::HuffmanTree;
use compressor::encoding::huffman::HuffmanEncoder;
use criterion::{criterion_group, criterion_main, Criterion};

use std::fs::File;
use std::io::BufReader;

fn compressor_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Build Huffman Tree and encode text, using");

    let str_token_test_data = File::open("./test_data/Ulysses.txt").unwrap();
    let str_reader = BufReader::new(&str_token_test_data);
    let str_token_input = TokenParser::words_from_reader(str_reader);

    let char_token_test_data = File::open("./test_data/Ulysses.txt").unwrap();
    let char_reader = BufReader::new(&char_token_test_data);
    let char_token_input = TokenParser::chars_from_reader(char_reader);

    group.bench_function("word tokens", |b| {
        b.iter(|| {
            let tree = HuffmanTree::from_frequencies(&str_token_input.token_frequencies);
            let encoder = HuffmanEncoder::from_huffman_tree(tree);
            encoder.encode(&str_token_input.lines, |line| {
                line.split_inclusive(' ').map(String::from)
            });
        })
    });

    group.bench_function("char tokens", |b| {
        b.iter(|| {
            let tree = HuffmanTree::from_frequencies(&char_token_input.token_frequencies);
            let encoder = HuffmanEncoder::from_huffman_tree(tree);
            encoder.encode(&str_token_input.lines, |line| line.chars());
        })
    });
}

criterion_group!(benches, compressor_benchmark);
criterion_main!(benches);
