use crate::ast::{Node, NodeType};
use crate::tokens::{Token, TokenType};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
    pub ast: Node,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
            ast: Node::new(NodeType::Program, String::from("")),
        }
    }
}