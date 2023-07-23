# Huffman coding

An implementation of Huffman coding for the ascii charset.

## What is Huffman coding?

Each ascii character uses 8 bits (1 byte). The principal behind Huffman coding is to assign variable length bit strings to represent different patterns in the text. More common patterns are allocated shorter bit strings, hence reducing the overall size of the string. 

An important detail, the encoding for any pattern must not be a prefix for any other encoding. Otherwise, there may be ambiguity when decoding the encoded string.

>For instance, if a=0, b=1, c=10, then 010 could be either: "aba" or "ac"
>
>Whereas if a = 0, b=10, c=11, then 0110 is always "aca", 01011011011 is "abcacac" 

## What is the algorithm?

Encoding:

1) Create the Huffman tree.
	- This is a binary prefix tree that contains the common patterns in the text.
	- We construct the tree such that each leaf represents a pattern (in this implementation, a char or a string).
	- The encoding for each pattern is derived from the path from the root to the leaf. Each right traversal appends 1 to encoded string, left appends 0.
	- We aim to construct a tree where the most frequent nodes have the shortest path from the root, hence the shortest encoding.
	- In this implementation, we focus on encoding individual letters and ignore the potential of multiple character patterns.

2) Use tree to create map of character -> encoding + vice versa.

3) Use map of character->encoding to create encoded string.

4) Share encoded string and the map from encoding->character.

Decoding:

1) Use the supplied map to decode the encoded string.
