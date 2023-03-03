use crate::tokens::{Token, TokenType};

use std::*;

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
            tokens: Vec::<Token>::new(),
        }
    }

    pub fn lex(&mut self) {
        let lines = self.code.split("\n");
        let mut all_tokens = Vec::<Token>::new();
    
        for line in lines {
            let mut chars = line.chars().peekable();
    
            while let Some(cc) = chars.next() {
                match cc {
                    // Match current character to left and right parens and curly braces
                    '{' => all_tokens.push(Token::new(TokenType::L_CURLY, cc.to_string())),
                    '}' => all_tokens.push(Token::new(TokenType::R_CURLY, cc.to_string())),
                    '(' => all_tokens.push(Token::new(TokenType::L_PAREN, cc.to_string())),
                    ')' => all_tokens.push(Token::new(TokenType::R_PAREN, cc.to_string())),
    
                    '+' | '-' | '*' | '/' | '=' => all_tokens.push(Token::new(TokenType::OPERATOR,cc.to_string())),
    
                    c if c.is_ascii_digit() => {
                        let mut num = String::new();
                        num.push(c);
    
                        while let Some(&cc) = chars.peek() {
                            if cc.is_ascii_digit() || cc == '.' {
                                num.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
    
                        all_tokens.push(Token::new(TokenType::NUMBER, num));
                    }
    
                    // words to check for keywords and identifiers
                    c if c.is_ascii_alphabetic() || cc == '_' => {
                        let mut keyw = String::new();
                        keyw.push(c);
    
                        while let Some(&cc) = chars.peek() {
                            if cc.is_ascii_alphabetic() || cc == '_' || cc.is_ascii_digit() {
                                keyw.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
    
                        // check for reserved keywords, otherwise identifier
    
                        match keyw.as_str() {
                            "let" => all_tokens.push(Token::new(TokenType::KEYWORD, keyw)),
                            "circle" => all_tokens.push(Token::new(TokenType::SHAPE_KEYWORD, keyw)),
                            "triangle" => all_tokens.push(Token::new(TokenType::SHAPE_KEYWORD, keyw)),
                            "shift" => all_tokens.push(Token::new(TokenType::SHIFT_KEYWORD, keyw)),
                            "stretch" => all_tokens.push(Token::new(TokenType::STRETCH_KEYWORD, keyw)),
                            "rotate" => all_tokens.push(Token::new(TokenType::ROTATE_KEYWORD, keyw)),
                            "evolve" => all_tokens.push(Token::new(TokenType::EVOLVE_KEYWORD, keyw)),
                            "generations" => all_tokens.push(Token::new(TokenType::KEYWORD, keyw)),
                            "radius" => all_tokens.push(Token::new(TokenType::KEYWORD, keyw)),
    
                            _ => all_tokens.push(Token::new(TokenType::IDENTIFIER, keyw))
                        }
                    }
    
                    _ => ()
                }
            }
        }
        
        self.tokens = all_tokens;
    }
}

