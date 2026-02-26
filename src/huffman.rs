use std::collections::HashMap;

use crate::tree::{Node, Tree};

pub struct Huffman;

impl Huffman {
    pub fn encode(&self, data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let mut freq: HashMap<u8, u64> = HashMap::new();
        for &b in data {
            *freq.entry(b).or_insert(0) += 1;
        }

        let tree = Tree::from_freq(&freq);
        let tree_bytes = tree.serialize();

        let mut bits = Vec::new();
        for &b in data {
            bits.extend_from_slice(&tree.codes[&b]);
        }
        let encoded = pack_bits(&bits);

        let mut out = Vec::new();
        out.extend_from_slice(&(data.len() as u64).to_be_bytes());
        out.extend_from_slice(&tree_bytes);
        out.extend_from_slice(&encoded);
        out
    }

    // format: [orig_len: u64 BE][tree][encoded bits]
    pub fn decode(&self, data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let n = u64::from_be_bytes(data[0..8].try_into().unwrap()) as usize;
        let mut pos = 8;
        let tree = Tree::from_bytes(data, &mut pos);

        let mut out = Vec::with_capacity(n);
        let mut cur = &tree.root;

        for &byte in &data[pos..] {
            for i in 0..8 {
                let bit = (byte >> i) & 1 == 1;
                cur = match cur {
                    Node::Branch { left, right } => {
                        if bit {
                            right
                        } else {
                            left
                        }
                    }
                    _ => panic!(),
                };
                if let Node::Leaf { byte } = cur {
                    out.push(*byte);
                    if out.len() == n {
                        return out;
                    }
                    cur = &tree.root;
                }
            }
        }

        out
    }
}

fn pack_bits(bits: &[bool]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for i in (0..bits.len()).step_by(8) {
        let mut b = 0u8;
        for j in 0..8 {
            if i + j < bits.len() && bits[i + j] {
                b |= 1 << j;
            }
        }
        bytes.push(b);
    }
    bytes
}
