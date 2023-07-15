use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, PartialEq, Eq)]
pub enum Tree<T: Clone> {
    Leaf {
        token: T,
        count: u32,
    },
    InternalNode {
        count: u32,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}

impl<T: Clone + Eq> Ord for Tree<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.get_count().cmp(&self.get_count())
    }
}

impl<T: Clone + Eq> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Clone + Eq> Tree<T> {
    pub fn from_frequencies(counts: &HashMap<T, u32>) -> Box<Tree<T>> {
        println!("Building Huffman Tree using token frequency map");
        let mut heap = BinaryHeap::new();
        for (key, value) in counts.iter() {
            heap.push(Box::new(Tree::Leaf {
                token: key.clone(),
                count: value.clone(),
            }));
        }

        while heap.len() > 1 {
            let smaller_node = heap.pop().unwrap();
            let larger_node = heap.pop().unwrap();
            let parent_node = Box::new(Tree::InternalNode {
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
            Tree::Leaf { count, .. } => *count,
            Tree::InternalNode { count, .. } => *count,
        }
    }
}
