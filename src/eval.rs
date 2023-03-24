use std::collections::*;
use crate::ast::*;
use crate::art::{Circle, Rectangle, Polygon, SVG};
use crate::tokens::{Token, TokenType};

pub struct Interpreter {
    pub ast: ProgramNode,
    pub symbol_table: HashMap<String, Value>,
    pub call_stack: Vec<Node>,
}

pub enum Value {
    Number(f64),
    String(String),
    Tuple(Vec<Value>),
    Shape(Shapes),
    Statements(Vec<Node>),
}

pub enum Shapes {
    Circle(Circle),
    Rectangle(Rectangle),
    Polygon(Polygon),
    SVG(SVG),
}

impl Interpreter {
    pub fn new(ast: ProgramNode) -> Interpreter {
        Interpreter {
            ast: ast,
            symbol_table: HashMap::new(),
            call_stack: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        // iterate through statements in ast
        let statements = self.ast.statements.clone();

        for statement in statements {
            match statement {
                Node::Shape(shape) => {
                    // add shape to symbol table
                    self.symbol_table.insert(shape.name.clone(), Value::Statements(shape.statements));
                },
                Node::Statement(statement) => {
                    match statement.kind {
                        StatementKind::DrawShape(name, properties) => {
                            // add shape to symbol table
                        },
                        StatementKind::Expression(expression) => {
                            // evaluate expression
                        },
                        StatementKind::Return(expression) => {
                            // evaluate expression
                        },
                        StatementKind::Shift(expression_a, expression_b) => {
                            // evaluate expression
                            
                        },
                        StatementKind::Stretch(expression_a, expression_b) => {
                            // evaluate expression
                        },
                        StatementKind::Rotate(expression) => {
                            // evaluate expression
                        },
                    }
                },
                _ => {}
            }
        }

        
    }
}