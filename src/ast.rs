use crate::tokens::{Token, TokenType};

pub struct Node {
    pub value: String,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(value: String) -> Node {
        Node {
            value: value,
            children: Vec::new(),
        }
    }
}

