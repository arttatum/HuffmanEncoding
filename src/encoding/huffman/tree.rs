use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, PartialEq, Eq)]
pub enum HuffmanTree<T: Clone> {
    Leaf {
        token: T,
        count: u32,
    },
    InternalNode {
        count: u32,
        left: Box<HuffmanTree<T>>,
        right: Box<HuffmanTree<T>>,
    },
}

impl<T: Clone + Eq> Ord for HuffmanTree<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.get_count().cmp(&self.get_count())
    }
}

impl<T: Clone + Eq> PartialOrd for HuffmanTree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Clone + Eq> HuffmanTree<T> {
    pub fn from_frequencies(counts: &HashMap<T, u32>) -> Box<HuffmanTree<T>> {
        println!("Building Huffman HuffmanTree using token frequency map");
        let mut heap = BinaryHeap::new();
        for (key, value) in counts.iter() {
            heap.push(Box::new(HuffmanTree::Leaf {
                token: key.clone(),
                count: value.clone(),
            }));
        }

        while heap.len() > 1 {
            let smaller_node = heap.pop().unwrap();
            let larger_node = heap.pop().unwrap();
            let parent_node = Box::new(HuffmanTree::InternalNode {
                count: (*smaller_node).get_count() + (*larger_node).get_count(),
                left: smaller_node,
                right: larger_node,
            });
            heap.push(parent_node);
        }

        heap.pop().unwrap()
    }

    fn get_count(&self) -> u32 {
        match self {
            HuffmanTree::Leaf { count, .. } => *count,
            HuffmanTree::InternalNode { count, .. } => *count,
        }
    }
}
