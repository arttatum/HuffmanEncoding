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

compress-a-file:
	cargo build --release
	cat	./test_data/test.txt | ./target/release/compressor
	./target/release/compressor -m decompress

	diff ./test_data/test.txt /tmp/huffman_decoded.txt -s
	lsd --blocks=size ./test_data/test.txt
	lsd --blocks=size /tmp/compressed_huff.mv

compress-a-book:
	cargo build --release
	cat	./test_data/Ulysses.txt | ./target/release/compressor
	
	./target/release/compressor -m decompress

	diff ./test_data/Ulysses.txt /tmp/huffman_decoded.txt -s
	lsd --blocks=size ./test_data/Ulysses.txt
	lsd --blocks=size /tmp/compressed_huff.mv

clean:
	rm /tmp/huffman_decoded.txt /tmp/compressed_huff.mv
