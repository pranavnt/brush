use std::collections::*;
use crate::ast::{Node};
use crate::tokens::{Token, TokenType};

pub struct Interpreter {
    pub ast: Node,
    pub symbol_table: HashMap<String, Node>,
    pub call_stack: Vec<Node>,
}

impl Interpreter {
    pub fn new(ast: Node) -> Interpreter {
        Interpreter {
            ast: ast,
            symbol_table: HashMap::new(),
            call_stack: Vec::new(),
        }
    }

    pub fn run(&self) {
        // iterate through statements

        // shape declaration statements

        // shape draw statements
    }

    pub fn eval(&mut self, node: Node) {
        match node {

        }
    }
}