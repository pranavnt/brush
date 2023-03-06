use crate::ast::{Node, NodeType};
use crate::tokens::{Token, TokenType};

pub struct Interpreter {
    pub ast: Node,
}

impl Interpreter {
    pub fn new(ast: Node) -> Interpreter {
        Interpreter {
            ast: ast,
        }
    }

    pub fn run(&self) {
        println!("Interpreting AST");
    }
}