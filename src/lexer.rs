use crate::tokens::{Token, TokenType};

use std::*;

pub struct Lexer {
    code: String,
    position: i32,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer {
            code: code,
            position: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let lines = self.code.split("\n");
        let mut all_tokens = Vec::<Token>::new();
    
        for line in lines {
            let mut chars = line.chars().peekable();
    
            while let Some(cc) = chars.next() {
                match cc {
                    // Match current character to left and right parens and curly braces
                    ',' => all_tokens.push(Token::new(TokenType::COMMA, cc.to_string())),
                    '{' => all_tokens.push(Token::new(TokenType::L_CURLY, cc.to_string())),
                    '}' => all_tokens.push(Token::new(TokenType::R_CURLY, cc.to_string())),
                    '(' => all_tokens.push(Token::new(TokenType::L_PAREN, cc.to_string())),
                    ')' => all_tokens.push(Token::new(TokenType::R_PAREN, cc.to_string())),
    
                    '/' => {
                        if let Some('/') = chars.peek() {
                            break; // Skip processing the rest of the line
                        } else {
                            all_tokens.push(Token::new(TokenType::OPERATOR, cc.to_string()));
                        }
                    }

                    '+' | '-' | '*' | '=' => all_tokens.push(Token::new(TokenType::OPERATOR,cc.to_string())),
                    
                    //check for strings
                    '"' => {
                        let mut keyw = String::new();

                        // ignore refutability warning, will either break or panic
                        while let tc = chars.peek() {
                            if let Some(&cc) = tc {
                                if cc == '"' {
                                    chars.next();
                                    break;
                                } else {
                                    keyw.push(chars.next().unwrap());
                                }
                            }
                            else {
                                // fix this later, should be error because mismatched quotes
                                panic!("missing end quote");
                            }
                        }

                        all_tokens.push(Token::new(TokenType::STRING, keyw));
                    }

                    // check for numbers
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
                            "let" => all_tokens.push(Token::new(TokenType::LET, keyw)),

                            "circle" => all_tokens.push(Token::new(TokenType::SHAPE_KEYWORD, keyw)),
                            // "triangle" => all_tokens.push(Token::new(TokenType::SHAPE_KEYWORD, keyw)),
                            "rectangle" => all_tokens.push(Token::new(TokenType::SHAPE_KEYWORD, keyw)),
                            "polygon" => all_tokens.push(Token::new(TokenType::SHAPE_KEYWORD, keyw)),

                            "shift" => all_tokens.push(Token::new(TokenType::SHIFT_KEYWORD, keyw)),
                            "hue_shift" => all_tokens.push(Token::new(TokenType::HUE_SHIFT_KEYWORD, keyw)),
                            "stretch" => all_tokens.push(Token::new(TokenType::STRETCH_KEYWORD, keyw)),
                            "rotate" => all_tokens.push(Token::new(TokenType::ROTATE_KEYWORD, keyw)),
                            "rotate_to" => all_tokens.push(Token::new(TokenType::ROTATETO_KEYWORD, keyw)),
                            "rotate_about" => all_tokens.push(Token::new(TokenType::ROTATEABOUT_KEYWORD, keyw)),
                            "warp" => all_tokens.push(Token::new(TokenType::WARP_KEYWORD, keyw)),
                            "evolve" => all_tokens.push(Token::new(TokenType::EVOLVE_KEYWORD, keyw)),
                            "reflect" => all_tokens.push(Token::new(TokenType::REFLECT_KEYWORD, keyw)),

                            "true" => all_tokens.push(Token::new(TokenType::BOOLEAN, keyw)),
                            "false" => all_tokens.push(Token::new(TokenType::BOOLEAN, keyw)),
                            _ => all_tokens.push(Token::new(TokenType::IDENTIFIER, keyw))
                        }
                    }
    
                    _ => ()
                }
            }

            // only push endline token if the token is not a curly brace
            if !line.contains("{") && !line.contains("}") {
                all_tokens.push(Token::new(TokenType::ENDLINE, String::from("")))
            }
        }
        
        all_tokens
    }
}

