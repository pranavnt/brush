use crate::ast::{Node, NodeType};
use crate::tokens::{Token, TokenType};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Node {
        let mut root = Node::new(NodeType::Program, String::from("Program"));


        // all parser code 

        root
    }
}