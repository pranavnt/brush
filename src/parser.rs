use std::collections::HashSet;
use crate::ast::*;
use crate::tokens::{Token,TokenType};

pub struct Parser {
    pub tokens: Vec<Token>,
    current: i64,
    shapes: HashSet<String>,

}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
            shapes: HashSet::<String>::new(),
        }
    }

    pub fn parse_program(&mut self) -> ProgramNode {
        let mut program = ProgramNode {
            statements: vec![],
        };

        // if we see a let, it's a shape declaration
        // if we see a function call to a "my_shape" (or function name), it's a shape draw
        while self.current < self.tokens.len() as i64 {
            let token = &self.tokens[self.current as usize];

            match token.token_type {
                TokenType::LET => {
                    // calculate end position based on when the next closing curly brace is
                    let mut end_pos = self.current + 1;
                    while self.tokens[end_pos as usize].token_type != TokenType::R_CURLY {
                        end_pos += 1;
                    }

                    let (shape, name) = self.parse_shape_declaration(end_pos);
                    
                    self.shapes.insert(name);

                    println!("{:?}", shape);

                    program.statements.push(shape);
                }

                TokenType::IDENTIFIER => {
                    let mut end_pos = self.current;

                    let mut paren_count = 1;

                    while paren_count > 0 {
                        if self.tokens[end_pos as usize].token_type == TokenType::L_PAREN {
                            paren_count += 1;
                        } else if self.tokens[end_pos as usize].token_type == TokenType::R_PAREN {
                            paren_count -= 1;
                        }

                        println!("paren count: {}", paren_count);

                        end_pos += 1;
                    }

                    let shape = self.parse_shape_draw(end_pos);
                    program.statements.push(shape);
                }

                _ => {
                    if token.token_type == TokenType::ENDLINE || token.token_type == TokenType::R_CURLY {
                        self.current += 1;
                        continue;
                    }
                    let statement = self.parse_statement(self.current + 1);
                    program.statements.push(statement);
                }
            }
        }                

        return program;
    }

    pub fn parse_shape_declaration(&mut self, end_pos: i64) -> (Node, String) {
        self.advance_past(TokenType::LET);
        let name = self.tokens[self.current as usize].value.clone();
        
        self.advance_past(TokenType::OPERATOR);
        let shape_kind = self.tokens[self.current as usize].value.clone();

        self.advance_past(TokenType::L_CURLY);
        
        let mut statements = vec![];

        
        while self.current < end_pos {
            let final_pos = self.get_next(TokenType::ENDLINE);
            let statement = self.parse_statement(final_pos);
            statements.push(statement);
        }

        let shape = ShapeNode {
            kind: match shape_kind.as_str() {
                "circle" => ShapeKind::Circle,
                "square" => ShapeKind::Rectangle,
                "svg" => ShapeKind::SVG,
                "polygon" => ShapeKind::Polygon,
                _ => {
                    panic!("Invalid shape type");
                },
            },
            statements: statements,
        };

        return (Node::Shape(shape), name);
    }

    pub fn parse_shape_draw(&mut self, end_pos: i64) -> Node {
        let name = self.tokens[self.current as usize].value.clone();

        if !self.shapes.contains(&name) {
            panic!("Shape {} not defined", name);
        }

        self.advance_past(TokenType::L_PAREN);

        let mut properties = vec![];

        while self.current < end_pos {
            let property_name = self.tokens[self.current as usize].value.clone();
            self.advance_past(TokenType::OPERATOR);
            let property_value = self.parse_expression(self.get_next(TokenType::ENDLINE));

            let property_node = PropertyNode {
                name: property_name,
                value: Box::new(property_value),
            };
            properties.push(property_node);
        }

        return Node::Statement(StatementNode {
            kind: StatementKind::DrawShape(name, properties),
        });
    }

    pub fn parse_statement(&mut self, end_pos: i64) -> Node {
        // statements are always going to fall into the following categories
        // 1. tranformation (warp, rotate, stretch, shift)
        // 2. property modification (radius, etc.)

        match self.tokens[self.current as usize].token_type {
            TokenType::SHIFT_KEYWORD => {
                self.advance_past(TokenType::SHIFT_KEYWORD);

                // parse x
                self.advance_past(TokenType::L_PAREN);
                let x = self.parse_expression(self.get_next(TokenType::COMMA));

                self.advance_past(TokenType::COMMA);
                let y = self.parse_expression(self.get_next(TokenType::R_PAREN));

                self.advance_past(TokenType::ENDLINE);

                return Node::Statement(StatementNode {
                    kind: StatementKind::Shift(Box::new(x), Box::new(y)),
                });
            },    
            TokenType::ROTATE_KEYWORD => {
                self.advance_past(TokenType::ROTATE_KEYWORD);

                // parse x
                self.advance_past(TokenType::L_PAREN);
                let angle = self.parse_expression(self.get_next(TokenType::R_PAREN));

                self.advance_past(TokenType::ENDLINE);

                return Node::Statement(StatementNode {
                    kind: StatementKind::Rotate(Box::new(angle)),
                });
            },
            TokenType::STRETCH_KEYWORD => {
                self.advance_past(TokenType::STRETCH_KEYWORD);

                // parse x
                self.advance_past(TokenType::L_PAREN);
                let x = self.parse_expression(self.get_next(TokenType::COMMA));

                self.advance_past(TokenType::COMMA);
                let y = self.parse_expression(self.get_next(TokenType::R_PAREN));

                self.advance_past(TokenType::ENDLINE);

                return Node::Statement(StatementNode {
                    kind: StatementKind::Stretch(Box::new(x), Box::new(y)),
                });
            },
            TokenType::KEYWORD => {
                let keyword = self.tokens[self.current as usize].value.clone();

                self.advance_past(TokenType::KEYWORD);

                let expression = self.parse_expression(end_pos);

                unimplemented!();
            },
            _ => {
                panic!("Invalid statement");
            }
        }
    }

    pub fn parse_expression(&mut self, end_pos: i64) -> Node {
        // expressions will always be:
        // direct value (number, string, coordinate, etc.)
        // math expression (add, subtract, multiply, divide)
        // boolean expression (and, or, not)
        // comparison expression (equal, greater than, less than, etc.)
        // NO function calls, function calls are statements

        // check if there are operators
        let mut bool_operators = false;
        let mut math_operators = false;

        let mut i = self.current;

        while i < end_pos {
            let token = &self.tokens[i as usize];

            match token.token_type {
                TokenType::OPERATOR => {
                    match token.value.as_str() {
                        "+" | "-" | "*" | "/" => {
                            math_operators = true;
                        },
                        "and" | "or" | "not" => {
                            bool_operators = true;
                        },
                        _ => {
                            // do nothing
                        }
                    }
                },
                _ => {
                    // do nothing
                }
            }

            i += 1;
        }

        if bool_operators && math_operators {
            panic!("Cannot mix boolean and math operators");
        } else if bool_operators {
            return self.parse_boolean_expression(self.current.clone(), end_pos);
        } else if math_operators {
            return self.parse_math_expression(self.current.clone(), end_pos);
        } else {
            // check that there's only one token
            if self.current + 1 != end_pos {
                // it's probably a coordinate OR invalid
                if self.tokens[self.current as usize].token_type == TokenType::L_PAREN {
                    // it's a coordinate
                    let x = self.parse_expression(self.get_next(TokenType::COMMA));
                    self.advance_past(TokenType::COMMA);
                    let y = self.parse_expression(self.get_next(TokenType::R_PAREN));
                    self.advance_past(TokenType::R_PAREN);

                    return Node::TupleLiteral(TupleLiteralNode {
                        values: vec![x, y],
                    });
                } else {
                    panic!("Invalid expression");
                }
            }

            let token = self.tokens[self.current as usize].clone();

            match token.token_type {
                TokenType::NUMBER => {
                    self.advance_past(TokenType::NUMBER);

                    return Node::NumberLiteral(NumberLiteralNode {
                        value: token.value.parse::<f64>().unwrap(),
                    });
                },
                _ => {
                    panic!("Invalid expression");
                }
            }
        }
    }

    fn parse_boolean_expression(&mut self, start_pos: i64, end_pos: i64) -> Node {
        let mut i = self.current;

        while i < end_pos {
            let token = &self.tokens[i as usize];

            // set left to being whatever is left of the operation with highest precedence
            if token.token_type == TokenType::OPERATOR && token.value == "not" {
                let left = self.parse_boolean_expression(start_pos, i);
                let right = self.parse_boolean_expression(i + 1, end_pos);

                return Node::BinaryExpression(BinaryExpressionNode {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator: BinaryOperator::Not,
                });
            } else if token.token_type == TokenType::OPERATOR && token.value == "and" {
                let left = self.parse_boolean_expression(start_pos, i);
                let right = self.parse_boolean_expression(i + 1, end_pos);

                return Node::BinaryExpression(BinaryExpressionNode {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator: BinaryOperator::And,
                });
            } else if token.token_type == TokenType::OPERATOR && token.value == "or" {
                let left = self.parse_boolean_expression(start_pos, i);
                let right = self.parse_boolean_expression(i + 1, end_pos);

                return Node::BinaryExpression(BinaryExpressionNode {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator: BinaryOperator::Or,
                });
            } else {
                i += 1;
            }
        }

        panic!("Invalid boolean expression");
    }

    fn parse_math_expression(&mut self, start_pos: i64, end_pos: i64) -> Node {
        // * or /, then + or -
        let mut i = self.current;

        while i < end_pos {
            let token = &self.tokens[i as usize];

            // set left to being whatever is left of the operation with highest precedence
            if token.token_type == TokenType::OPERATOR && token.value == "*" {
                let left = self.parse_math_expression(start_pos, i);
                let right = self.parse_math_expression(i + 1, end_pos);

                return Node::BinaryExpression(BinaryExpressionNode {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator: BinaryOperator::Multiply,
                });
            } else if token.token_type == TokenType::OPERATOR && token.value == "/" {
                let left = self.parse_math_expression(start_pos, i);
                let right = self.parse_math_expression(i + 1, end_pos);

                return Node::BinaryExpression(BinaryExpressionNode {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator: BinaryOperator::Divide,
                });
            } else if token.token_type == TokenType::OPERATOR && token.value == "+" {
                let left = self.parse_math_expression(start_pos, i);
                let right = self.parse_math_expression(i + 1, end_pos);

                return Node::BinaryExpression(BinaryExpressionNode {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator: BinaryOperator::Plus,
                });
            } else if token.token_type == TokenType::OPERATOR && token.value == "-" {
                let left = self.parse_math_expression(start_pos, i);
                let right = self.parse_math_expression(i + 1, end_pos);

                return Node::BinaryExpression(BinaryExpressionNode {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator: BinaryOperator::Minus,
                });
            } else {
                i += 1;
            }
        }

        panic!("Invalid math expression");
    }

    fn advance_past(&mut self, token_type: TokenType) {
        while self.current < self.tokens.len() as i64 {
            let token = &self.tokens[self.current as usize];

            if token.token_type == token_type {
                self.current += 1;
                return;
            }

            self.current += 1;
        }
    }

    fn get_next(&self, kind: TokenType) -> i64 {
        let mut i = self.current + 1;

        while i < self.tokens.len() as i64 {
            let token = &self.tokens[i as usize];

            if token.token_type == kind {
                return i;
            }

            i += 1;
        }

        panic!("Could not find token of type {:?}", kind);
    }
}
