use std::collections::*;
use crate::ast::*;
use crate::art::{Circle, Rectangle, Polygon, SVG};
use crate::tokens::{Token, TokenType};

pub struct Interpreter {
    pub ast: ProgramNode,
    pub symbol_table: HashMap<String, Value>,
    pub call_stack: Vec<Node>,
    pub shapes: Vec<Shapes>,
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
            shapes: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        // iterate through statements in ast
        let statements = self.ast.statements.clone();

        for statement in statements {
            self.eval(statement);
        }
    }

    fn eval(&mut self, statement: Node) {
        match statement {
            Node::Shape(shape) => {
                // add shape to symbol table
                self.symbol_table.insert(shape.name.clone(), Value::Statements(shape.statements));
            },
            Node::Statement(statement) => {
                match statement.kind {
                    StatementKind::DrawShape(name, properties) => {
                        // check type of shape
                        let shape = self.symbol_table.get(&name).unwrap();

                        // check if shape is circle, rectangle, polygon, or svg
                        match shape {
                            Value::Shape(Shapes::Circle(circle)) => {
                                let mut generations = 1;
                                let mut radius = 1.0;
                                let mut fill = (0, 0, 0);
                                let mut outline_color = (0, 0, 0);
                                let mut outline_width = 1.0;
                                
                                // evaluate properties
                                for property in properties {
                                    if property.name == "generations" {
                                        generations = match *property.value {
                                            Node::NumberLiteral(NumberLiteralNode { value }) => value as i32,
                                            _ => 1,
                                        }
                                    } else if property.name == "radius" {
                                        radius = match *property.value {
                                            Node::NumberLiteral(NumberLiteralNode { value }) => value,
                                            _ => 1.0,
                                        }
                                    }
                                    // other properties
                                }

                                let circle = Circle::new();

                                self.shapes.push(Shapes::Circle(circle));

                                for _ in 0..generations {
                                    // clone circle
                                    // circle = circle.clone();

                                    

                                    // push to shapes

                                }
                            },
                            Value::Shape(Shapes::Rectangle(rectangle)) => {
                                unimplemented!();
                            },
                            Value::Shape(Shapes::Polygon(polygon)) => {
                                unimplemented!();
                            },
                            Value::Shape(Shapes::SVG(svg)) => {
                                unimplemented!();
                            },
                            _ => {}
                        }
                    },
                    StatementKind::Expression(expression) => {
                        // evaluate expression
                        
                    },
                    StatementKind::Shift(x, y) => {
                        // evaluate expressions that make up x and y
                    },
                    StatementKind::Stretch(x, y) => {
                        // evaluate expression
                    },
                    StatementKind::Rotate(angle) => {
                        // evaluate expression
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}