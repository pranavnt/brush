use std::collections::*;
use crate::ast::*;
use crate::art::{Circle, Rectangle, Polygon, SVG, Shape};
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
    pub fn run(&mut self) {
        for statements in self.ast.statements.iter() {
            self.eval(statements);
        }
    }

    pub fn eval(&mut self, node: &Node) -> Value {
        match node {
            Node::Program(program) => {
                for statement in program.statements.iter() {
                    self.eval(statement);
                }
                Value::Statements(program.statements.clone())
            }

            Node::Statement(statement) => match statement.kind {
                StatementKind::DrawShape(name, properties) => {
                    unimplemented!()
                },
                StatementKind::Expression(expression) => self.eval(&expression),
                StatementKind::Return(expression) => self.eval(&expression),
                StatementKind::Shift(x,y) => {
                    let x_val = self.eval(&x);
                    let y_val = self.eval(&y);
                    // shift the shape here // 
                    Value::Tuple(vec![x_val, y_val])
                },
                StatementKind::Stretch(x, y) => {
                    let x_val = self.eval(&x);
                    let y_val = self.eval(&y);
                    // stretch the shape here //
                    Value::Tuple(vec![x_val, y_val])
                },
                StatementKind::Rotate(angle) => {
                    let angle_val = self.eval(&angle);
                    // rotate the shape here //
                    angle_val
                },

            },

            Node::Identifier(identifier) => {
                let name = &identifier.name;
                match self.symbol_table.get(name) {
                    Some(value) => unimplemented!(), //value.clone(),
                    None => panic!("Undefined variable: {}", name),
                }
            },

            Node::Shape(Shape) => {
                unimplemented!()
            },
            Node::BinaryExpression(expression) => {
                unimplemented!()
            }
            Node::Block(expression) => {
                unimplemented!()
            }
            Node::Property(expression) => {
                unimplemented!()
            },
            Node::NumberLiteral(expression) => {
                unimplemented!()
            }
            Node::StringLiteral(expression) => {
                unimplemented!()
            }
            Node::BooleanLiteral(expression) => {
                unimplemented!()
            },
            Node::TupleLiteral(expression) => {
                unimplemented!()
            },
            Node::FunctionCall(expression) => {
                unimplemented!()
            },
            Node::VariableDeclaration(expression) => {
                unimplemented!()
            },
            Node::VariableAssignment(expression) => {
                unimplemented!()
            },
            Node::IfStatement(expression) => {
                unimplemented!()
            },
            Node::WhileLoop(expression) => {
                unimplemented!()
            },
            Node::ForLoop(expression) => {
                unimplemented!()
            },


            
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    /* pub fn new(ast: ProgramNode) -> Interpreter {
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

        
    } */

    
}