# Huffman coding

A text {de,}compression tool inspired by the Unix philosophy.

## Quick Start

You will need:
- [Cargo & rustc](https://www.rust-lang.org/tools/install)
- (Optional, to run recipes) [Just](https://github.com/casey/just/tree/master); like GNU Make, but just a command runner
- A UNIX shell

To compress the book Ulysses with huffman encoding, run:

`just compress-a-book`

On my computer it takes 7ms.

## How do I use the CLI?

Run `just release` to run the tests and generate an optimised executable in the `$PROJECT_ROOT/target/release/` directory.

Alternatively, run `cargo build --release`.

You can either run the executable from the `$PROJECT_ROOT/target/release/` directory or add it to your `$PATH`.

## What is Huffman coding?

Common text encoding schemes, such as ascii or utf-8, are not the most memory efficient encodings for the storage of text documents. In fact, given two arbitrary documents, their theoretical optimal encoding schemes are typically very different. Huffman coding is a memory optimised prefix encoding scheme for an individual document.

The principle behind Huffman coding is to assign variable length bit strings to represent different tokens (e.g. chars, words) in a document. More common tokens are allocated shorter bit string encodings, hence reducing the overall memory required to store the document in its Huffman encoded form. 

### Prefix Encoding

The bit string encoding for any token must not be a prefix of any other encoding, otherwise there is ambiguity when decoding the encoded string.

For example:

    Given an encoding scheme: a=0, b=1, c=10, b's encoding is a prefix to c's encoding.

    As a result, the encoded bit string 010 could be either: "aba" or "ac"
 
    A prefix encoding scheme such as: a=0, b=10, c=11, ensures no token's encoding prefixes any other. 
    
    The bit string: 0110 is deterministically decoded to "aca", 01011011011 to "abcacac", with no ambiguity. 

## What is the Huffman coding algorithm?

Typically, Huffman coding refers to creating an optimal prefix code for a document, in the form of a Huffman Tree.

An informal definition of a Huffman Tree is: (1) a binary prefix tree, (2) where leaves contain tokens (3) and leaves with the least depth contain the most common tokens in a document.

The tree can be used to encode the document into a memory efficient binary string. Similarly, the tree can be used to decode the encoded document.

To build the tree:

1) Iterate over tokens in the document to create a frequency map of tokens ([token: count])
2) Create an empty priority queue (min heap), where higher frequency => higher priority
3) For each item in the frequency map:
	
	a) Create a Huffman Leaf Node (token, count)

   	b) Add it to the priority queue

4) While the number of items in the priority queue is greater than one:
	
    a) Remove the top two items from the priority
   
    b) Create a new Huffman Tree Internal Node which:

	- References the lower frequency child

	- References the higher frequency child

	- Has a frequency field equal to the sum of both children's frequencies 
		
	- Add the new node to the priority queue

8) The single node remaining in the queue is the root of the Huffman Tree

## Intuition

More frequently occurring tokens are added to the Huffman Tree later in the construction process. When they are added, all items added previously (which have lower frequency) must have equal or greater depth in the resultant tree. As a result: 
- The least frequently occurring tokens have the greatest depth and the longest encodings; 
- The most frequently occurring tokens have the least depth and the shortest encodings.

This is beneficial, because it means we can assign less memory to encode common tokens.

## Encoding and Decoding

To deduce the mapping from original_token <-> encoded_token:

1) Traverse the Huffman Tree from the root node to the leaf that contains the token.
2) At each (right|left) traversal, a (0|1) is added to the candidate encoding.

In this solution, the encoded_token<->original_token relationships are saved into two maps to enable:
- Efficient encoding of large text files, without multiple traversals of the Tree.
- Serialisation of the encoding->original mapping, which is saved with the encoded text, then used by the tool during decompression.
