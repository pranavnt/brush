use crate::tokens::{Token, TokenType};

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Program,
    ShapeDeclaration,
    Shape(String),
    Property(String),
    DrawShape,
    MathExpression,
    NumberLiteral,
    Identifier,
    Operator,
    ShapeProperty,
    ShapeIdentifier
}

#[derive(Debug)]
pub struct ProgramNode{
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

#[derive(Debug)]
pub struct ShapeDeclarationNode{
    pub node_type: NodeType,
    pub identifier: String,
    pub children: Vec<Node>,
}

#[derive(Debug)]
pub struct ShapeNode{
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

#[derive(Debug)]
pub struct PropertyNode{
    pub node_type: NodeType,
    pub identifier: String,
    pub value: Node,
}

#[derive(Debug)]
pub struct DrawShapeNode{
    pub node_type: NodeType,
    pub identifier: String,
    pub properties: Vec<Node>,
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub value: Option<Token>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(node_type: NodeType, value: Option<Token>) -> Node {
        Node {
            node_type,
            value,
            children: vec![],
        }
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();

        match self.node_type {
            NodeType::Program => {
                for child in &self.children {
                    string.push_str(&child.to_string());
                }
            }
            NodeType::ShapeDeclaration => {
                string.push_str("ShapeDeclaration \n");
            }
            NodeType::Shape(ref shape_type) => {
                string.push_str("Shape \n");
            }
            NodeType::Property(ref identifier) => {
                string.push_str("Property \n");
            }
            NodeType::DrawShape => {
                string.push_str("DrawShape \n ");
            }
            NodeType::MathExpression => {
                string.push_str(&format!("MathExpression({}) \n", self.value.as_ref().unwrap().value));
            }
            NodeType::NumberLiteral => {
                string.push_str(&format!("NumberLiteral({}) \n", self.value.as_ref().unwrap().value));
            }
            NodeType::Identifier => {
                string.push_str(&format!("Identifier({}) \n", self.value.as_ref().unwrap().value));
            }
            NodeType::Operator => {
                string.push_str(&format!("Operator({}) \n", self.value.as_ref().unwrap().value));
            }
            NodeType::ShapeProperty => {
                string.push_str(&format!("ShapeProperty() \n"));
            }
            NodeType::ShapeIdentifier => {
                string.push_str(&format!("ShapeIdentifier({}) \n", self.value.as_ref().unwrap().value));
            }
        }

        string
    }
}