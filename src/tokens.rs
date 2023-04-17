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
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    T_PROGRAM,
    ENDLINE,
    L_CURLY,
    R_CURLY,
    L_PAREN,
    R_PAREN,
    IDENTIFIER,
    NUMBER,
    STRING,
    BOOLEAN,
    OPERATOR,
    LET,
    COMMA,
    KEYWORD,
    SHAPE_KEYWORD,
    SHIFT_KEYWORD,
    HUE_SHIFT_KEYWORD,
    STRETCH_KEYWORD,
    ROTATE_KEYWORD,
    REFLECT_KEYWORD,
    EVOLVE_KEYWORD,
    PROPERTIES,
}

#[derive(Debug, Clone)]
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
