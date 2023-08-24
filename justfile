release:
	just fmt
	cargo test
	cargo bench
	cargo build --release

fmt:
  find {{invocation_directory()}} -name \*.rs -exec rustfmt {} \;

build:
	cargo build

test:
	cargo test

bench:
	cargo bench

DEMO_OUT_FILE_PATH := "/tmp/just_a_compressed_file.mv"
DEMO_DECOMP_OUT_FILE_PATH := "/tmp/just_an_uncompressed_file.txt"

set positional-arguments

compress-a-file:
	just compress ./test_data/test.txt

compress-a-book:
	just compress ./test_data/Ulysses.txt


@compress file_path:
	cargo build --release
	cat	$1 | ./target/release/compressor --out-file {{ DEMO_OUT_FILE_PATH }} 
	echo "Compressed file and wrote it to {{ DEMO_OUT_FILE_PATH }}"	
	./target/release/compressor -m decompress --in-file {{ DEMO_OUT_FILE_PATH }} --out-file {{ DEMO_DECOMP_OUT_FILE_PATH }} 
	echo "Decompressed file and wrote it to {{ DEMO_DECOMP_OUT_FILE_PATH }}"	
	diff $1 {{ DEMO_DECOMP_OUT_FILE_PATH }} -s
	
clean:
	rm {{ DEMO_OUT_FILE_PATH }} {{ DEMO_DECOMP_OUT_FILE_PATH }} 
