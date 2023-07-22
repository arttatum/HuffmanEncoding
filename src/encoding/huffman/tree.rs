use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

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

#[test]
fn test_from_frequencies() {
    let lyrics = "Hi, my name is, what? My name is, who?
                        My name is, chka-chka, Slim Shady
                        Hi, my name is, huh? My name is, what?
                        My name is, chka-chka, Slim Shady";

    let counts = lyrics.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    let tree = HuffmanTree::from_frequencies(&counts);
    assert_eq!(tree.get_count(), u32::try_from(lyrics.len()).unwrap())
}

#[test]
fn test_char_leaves_have_correct_count() {
    let lyrics = "Hi, my name is, what? My name is, who?
                        My name is, chka-chka, Slim Shady
                        Hi, my name is, huh? My name is, what?
                        My name is, chka-chka, Slim Shady";

    let counts = lyrics.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    let tree = HuffmanTree::from_frequencies(&counts);

    assert_eq!(tree.get_count(), u32::try_from(lyrics.len()).unwrap());

    check_leaves(tree, counts);
}

#[test]
fn test_str_leaves_have_correct_count() {
    let lyrics = "Hi, my name is, what? My name is, who?
My name is, chka-chka, Slim Shady
Hi, my name is, huh? My name is, what?
My name is, chka-chka, Slim Shady";

    let counts = lyrics
        .split_inclusive(' ')
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });

    let tree = HuffmanTree::from_frequencies(&counts);

    assert_eq!(tree.get_count(), 27);

    check_leaves(tree, counts);
}

#[allow(dead_code)]
fn check_leaves<T>(tree: Box<HuffmanTree<T>>, counts: HashMap<T, u32>)
where
    T: Clone + Eq + Hash,
{
    match *tree {
        HuffmanTree::Leaf { count, token } => {
            assert_eq!(count, *counts.get(&token).unwrap())
        }
        HuffmanTree::InternalNode { left, right, .. } => {
            check_leaves(left, counts.clone());
            check_leaves(right, counts.clone());
        }
    }
}
