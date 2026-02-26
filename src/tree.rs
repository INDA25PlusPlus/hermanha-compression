use std::cmp::Reverse;
use std::collections::HashMap;

pub enum Node {
    Leaf { byte: u8 },
    Branch { left: Box<Node>, right: Box<Node> },
}

pub struct Tree {
    pub root: Node,
    pub codes: HashMap<u8, Vec<bool>>,
}

impl Tree {
    pub fn from_freq(freq: &HashMap<u8, u64>) -> Self {
        let root = build_tree(freq);
        let mut codes = HashMap::new();
        collect_codes(&root, &mut Vec::new(), &mut codes);
        Tree { root, codes }
    }

    pub fn from_bytes(data: &[u8], pos: &mut usize) -> Self {
        let root = parse_node(data, pos);
        Tree {
            root,
            codes: HashMap::new(),
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();
        write_node(&self.root, &mut out);
        out
    }
}

fn build_tree(freq: &HashMap<u8, u64>) -> Node {
    let mut nodes: Vec<(u64, Node)> = Vec::new();

    for (&byte, &count) in freq {
        nodes.push((count, Node::Leaf { byte }));
    }

    nodes.sort_by_key(|x| Reverse(x.0));

    if nodes.len() == 1 {
        let (_, n) = nodes.pop().unwrap();
        return Node::Branch {
            left: Box::new(n),
            right: Box::new(Node::Leaf { byte: 0 }),
        };
    }

    while nodes.len() > 1 {
        let (f1, n1) = nodes.pop().unwrap();
        let (f2, n2) = nodes.pop().unwrap();
        let merged = (
            f1 + f2,
            Node::Branch {
                left: Box::new(n1),
                right: Box::new(n2),
            },
        );
        let pos = nodes.partition_point(|x| x.0 > merged.0);
        nodes.insert(pos, merged);
    }

    nodes.pop().unwrap().1
}

fn collect_codes(node: &Node, prefix: &mut Vec<bool>, codes: &mut HashMap<u8, Vec<bool>>) {
    match node {
        Node::Leaf { byte } => {
            codes.insert(*byte, prefix.clone());
        }
        Node::Branch { left, right } => {
            prefix.push(false);
            collect_codes(left, prefix, codes);
            prefix.pop();

            prefix.push(true);
            collect_codes(right, prefix, codes);
            prefix.pop();
        }
    }
}

fn write_node(node: &Node, out: &mut Vec<u8>) {
    match node {
        Node::Leaf { byte } => {
            out.push(1);
            out.push(*byte);
        }
        Node::Branch { left, right } => {
            out.push(0);
            write_node(left, out);
            write_node(right, out);
        }
    }
}

fn parse_node(data: &[u8], pos: &mut usize) -> Node {
    let tag = data[*pos];
    *pos += 1;

    match tag {
        0 => {
            let left = parse_node(data, pos);
            let right = parse_node(data, pos);
            Node::Branch {
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        1 => {
            let b = data[*pos];
            *pos += 1;
            Node::Leaf { byte: b }
        }
        _ => panic!(),
    }
}
