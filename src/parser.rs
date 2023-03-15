use crate::ast::*;
use crate::tokens::{Token,};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: i64
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse_program(&mut self) {
        let mut program = ProgramNode {
            statements: vec![],
        };

        // parse statements that call other methods

        unimplemented!();
    }

    pub fn parse_statement(&mut self, parent: &mut Node) {
        // let mut statement = StatementNode {};

        // parse expression

        unimplemented!();
    }

    pub fn parse_expression(&mut self, parent: &mut Node) {
        // parse binary expression

        unimplemented!();
    }

    pub fn parse_shape_declaration(&mut self, parent: &mut Node) {
        // parse shape

        unimplemented!();
    }

    pub fn parse_shape_draw(&mut self, parent: &mut Node) {
        // parse draw_shape

        unimplemented!();
    }
}
