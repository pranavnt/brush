mod tokens;

use std::*;
use tokens::*;

pub struct Lexer {
    code: String,
    position: i32,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer {
            code: code,
            position: 0,
            tokens: Vec::Token::new(),
        }
    }
}