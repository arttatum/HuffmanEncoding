use compressor::application::input::Summary;
use compressor::encoding::huffman::tree::HuffmanTree;
use compressor::encoding::huffman::HuffmanEncoder;
use criterion::{criterion_group, criterion_main, Criterion};

use std::fs::File;
use std::io::BufReader;

fn compressor_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Build Huffman Tree and encode text, using");

    // Reading the whole book twice is grossly ineffecient. Alternatives, such as reading into
    // memory (Vec<_>) is a better idea typically.
    let str_token_test_data = File::open("./test_data/Ulysses.txt").unwrap();
    let str_reader = BufReader::new(&str_token_test_data);
    let str_token_input = Summary::strs_from_reader(str_reader);

    let char_token_test_data = File::open("./test_data/Ulysses.txt").unwrap();
    let char_reader = BufReader::new(&char_token_test_data);
    let char_token_input = Summary::chars_from_reader(char_reader);

    group.bench_function("word tokens", |b| {
        b.iter(|| {
            let tree = HuffmanTree::from_frequencies(&str_token_input.frequencies);
            let encoder = HuffmanEncoder::from_huffman_tree(tree);
            encoder.encode(
                str_token_input
                    .input
                    .split_inclusive(" ")
                    .map(|a| String::from(a))
                    .into_iter(),
            );
        })
    });

    group.bench_function("char tokens", |b| {
        b.iter(|| {
            let tree = HuffmanTree::from_frequencies(&char_token_input.frequencies);
            let encoder = HuffmanEncoder::from_huffman_tree(tree);
            encoder.encode(str_token_input.input.chars().into_iter());
        })
    });
}

criterion_group!(benches, compressor_benchmark);
criterion_main!(benches);
