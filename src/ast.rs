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
}