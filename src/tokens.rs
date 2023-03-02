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

pub enum TokenType {
    // go through the language and find all the tokens
    NUMBER,
    OPERATOR,
    SHIFT_KEYWORD,
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
    println!("{:#?}", lines.next());
}