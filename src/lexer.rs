use crate::tokens::{Token, TokenType, code_to_token};

use std::*;

pub struct Lexer {
    code: String,
    position: i32,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        code_to_token(&code);

        Lexer {
            code: code,
            position: 0,
            tokens: Vec::<Token>::new()
        }
    }
}