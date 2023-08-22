fmt:
  find {{invocation_directory()}} -name \*.rs -exec rustfmt {} \;

build:
	cargo build

test:
	cargo test

build-optimised:
	cargo build --release

bench:
	cargo bench

compress-a-book:
	cargo build --release
	cat	./test_data/Ulysses.txt | ./target/release/huffman
	diff ./test_data/Ulysses.txt /tmp/huffman_decoded.txt -s
