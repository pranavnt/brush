use crate::ast::{Node, NodeType};
use crate::tokens::{Token, TokenType};

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

    pub fn parse_expression(&mut self, parent: &mut Node) {
        // Create a new node for the head of the expression tree
        let mut node = Node::new(NodeType::MathExpression, None);
    
        // Add the first token as the value of the head node
        node.value = Some(self.tokens.remove(0));
    
        // Continue adding nodes to the expression tree until the end of the expression
        while self.tokens.len() > 0 && self.tokens.first().unwrap().token_type == TokenType::NUMBER || self.tokens.first().unwrap().token_type == TokenType::IDENTIFIER || self.tokens.first().unwrap().token_type == TokenType::OPERATOR {
            let token_type = &self.tokens.first().unwrap().token_type;
            let token_value = self.tokens.first().unwrap().value.clone();
    
            // Create a new node for the next token
            // let mut child_node = Node::new(NodeType::MathExpression, Some(Token::new(token_type, token_value)));
            let mut child_node = Node::new(NodeType::MathExpression, Some(self.tokens.first().unwrap().clone()));

            // Set the type of the child node based on the token type   
            match token_type {
                TokenType::NUMBER => child_node.node_type = NodeType::NumberLiteral,
                TokenType::IDENTIFIER => child_node.node_type = NodeType::Identifier,
                TokenType::OPERATOR => child_node.node_type = NodeType::Operator,
                _ => (),
            }
    
            // Add the child node to the head node
            node.children.push(child_node);
    
            // Remove the processed token from the token vector
            self.tokens.remove(0);
        }
    
        // Add the expression tree as a child of the parent node
        parent.children.push(node);
    } 
    
    pub fn parse_brackets(&mut self, parent: &mut Node) {
        // Iterate through until the end of brackets
        // until token's value is R curly
        while self.tokens.first().unwrap().token_type != TokenType::R_CURLY {
            self.parse_main(parent);
        }
        // Remove the R_CURLY token
        self.tokens.remove(0);
    }
    
    pub fn parse_paren(&mut self, parent: &mut Node) {
        // Iterate through until the end of paren
        while self.tokens.first().unwrap().token_type != TokenType::R_PAREN {
            self.parse_main(parent);
        }
        // Remove the R_PAREN token
        self.tokens.remove(0);
    }
    
    pub fn parse_main(&mut self, parent: &mut Node) {
        let token_type = (&self.tokens.first().unwrap().token_type).clone();
        let token_value = self.tokens.first().unwrap().value.clone();
        let mut node: Node = Node::new(NodeType::ShapeIdentifier, Some(Token::new(token_type, token_value)));
    
        if token_type == TokenType::ENDLINE {
            // Skip ENDLINE tokens
            self.tokens.remove(0);
        } else if token_type == TokenType::EVOLVE_KEYWORD {
            // Make a new child node of type ShapeDeclaration
            node.node_type = NodeType::ShapeDeclaration;
            parent.children.push(node);
            // Remove the EVOLVE_KEYWORD token
            self.tokens.remove(0);
        } else if token_type == TokenType::L_CURLY {
            // Parse brackets with parent as last child
            self.tokens.remove(0);
            if parent.children.last().unwrap().children.len() != 0 {
                self.parse_brackets(parent.children.last().unwrap().children.last_mut().unwrap());
            } else {
                self.parse_brackets(parent.children.last_mut().unwrap());
            }
        } else if token_type == TokenType::L_PAREN {
            // Parse parentheses with parent as last child
            self.tokens.remove(0);
            if parent.children.last().unwrap().children.len() != 0 {
                self.parse_paren(parent.children.last().unwrap().children.last_mut().unwrap());
            } else {
                self.parse_paren(parent.children.last_mut().unwrap());
            }
        } else if token_type == TokenType::PROPERTIES {
            // Make a new child node of type Property
            node.node_type = NodeType::ShapeProperty;
            parent.children.push(node);
            self.tokens.remove(0);
    
            if self.tokens.len() > 0 && self.tokens.first().unwrap().value == "=" {
                // Remove the equals sign
                self.tokens.remove(0);
                self.parse_expression(parent.children.last_mut().unwrap());
            }
        } else {
            // Otherwise, assume it's a ShapeIdentifier
            parent.children.push(node);
            self.tokens.remove(0);
        }
    }
    
    pub fn parse(&mut self) -> Node {
        let mut root = Node::new(NodeType::Program, Some(Token::new(TokenType::T_PROGRAM, String::from("Program"))));
    
        while self.tokens.len() > 0 {
            if self.tokens.first().unwrap().token_type == TokenType::L_CURLY {
                // Start a new tree from the last shape declaration or from root
                let mut parent: &mut Node = if root.children.len() > 0 && (root.children.last().unwrap().node_type == NodeType::ShapeDeclaration) {
                    root.children.last_mut().unwrap()
                } else {
                    &mut root
                };
                let mut shape_node = Node::new(NodeType::ShapeIdentifier, Some(Token::new(TokenType::IDENTIFIER, String::from("shape"))));
                shape_node.node_type = NodeType::ShapeDeclaration;
                parent.children.push(shape_node);
                parent = parent.children.last_mut().unwrap();
                // Parse the brackets
                self.tokens.remove(0);
                self.parse_brackets(parent);
            } else {
                // If it's not a L_CURLY token, assume it's a ShapeIdentifier
                let mut parent: &mut Node = if root.children.len() > 0 && root.children.last().unwrap().node_type == NodeType::ShapeDeclaration {
                    root.children.last_mut().unwrap()
                } else {
                    &mut root
                };
                self.parse_main(parent);
            }
        }
    
        return root;
    }
}
////old code -
// pub struct Parser {
//     pub tokens: Vec<Token>,
//     pub current: usize,
// }

// impl Parser {
//     pub fn new(tokens: Vec<Token>) -> Parser {
//         Parser {
//             tokens: tokens,
//             current: 0,
//         }
//     }

//     pub fn parse(&mut self) -> Node {
//         let mut root = Node::new(NodeType::Program, String::from("Program"));


//         // all parser code 

//         root
//     }

//     // additional functions might be 
// }