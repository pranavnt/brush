/* {
    type: "KEYWORD",
    value: "const"
}
---
{
    type: "OPERATOR",
    value: "+"
}
---
{
    type: "NUMBER",
    value: "2"
}*/
#[derive(Debug)]
pub enum TokenType {
    L_CURLY,
    R_CURLY,
    L_PAREN,
    R_PAREN,
    IDENTIFIER,
    NUMBER,
    OPERATOR,
    SHIFT_KEYWORD,
    STRETCH_KEYWORD,
    ROTATE_KEYWORDS
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Token {
        Token {
            token_type: token_type,
            value: value,
        }
    }
}

pub fn code_to_token(input: String) {
    let mut lines = input.split("\n");
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

                '+' | '-' | '*' | '/' => all_tokens.push(Token::new(TokenType::OPERATOR,cc.to_string())),

                _ => ()
            }
        }
    }

    for token in all_tokens {
        println!("{:?} : {}", token.token_type, token.value);
    }
}