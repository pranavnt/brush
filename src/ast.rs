use crate::tokens::{Token, TokenType};

// Different options for types of nodes
pub enum NodeType {
    Program,
    ShapeDeclaration,
    // More that are currently in notion
}

// 
pub struct Node {
    pub node_type: NodeType,
    pub value: String,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(node_type: NodeType, value: String) -> Node {
        Node {
            node_type: node_type,
            value: value,
            children: Vec::new(),
        }
    }
}