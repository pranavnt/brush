use std::collections::*;
use crate::ast::*;
use crate::art::{Circle, Rectangle, Polygon, SVG, Shape, Drawable};
use crate::tokens::{Token, TokenType};

pub struct Interpreter {
    pub ast: ProgramNode,
    pub symbol_table: HashMap<String, Value>,
    pub call_stack: Vec<Node>,

    pub shapes: Vec<Shape>,
}

#[derive(Debug)]
pub enum Value {
    Number(f32),
    String(String),
    Tuple(Vec<Value>),
    Shape(Shapes),
    Statements(Vec<Node>),
}

#[derive(Debug)]
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

            shapes: Vec::new()
        }
    }

    pub fn run(&mut self) {
        for statement in self.ast.statements.clone().iter() {
            self.tmp_eval(statement);
        }
    }

    pub fn tmp_eval(&mut self, node: &Node) -> Option<Value> {
        match node {
            Node::Program(program) => { // this code is never going to reach ???
                None
            }

            Node::Shape(shape) => {
                // println!("shape: {:#?}", shape);

                let name = &shape.name;
                match self.symbol_table.get(name) {
                    Some(value) => panic!("variable already declared with name: {}", name),

                    None => {
                        self.symbol_table.insert(name.clone(), Value::Shape(Shapes::Circle(Circle::new(0.0, 0.0, 1.0))));
                    }
                }

                None
            },

            Node::Statement(statement) => match statement.clone().kind {
                StatementKind::DrawShape(name, properties) => {
                    println!("{:#?}\n\n", statement);

                    match self.symbol_table.get(&name) {
                        Some(value) => {
                            match value {
                                Value::Shape(shape) => {
                                    match shape {
                                        Shapes::Circle(circle) => {
                                            for prop in &properties {
                                                match prop.name.as_str() {
                                                    "radius" => {
                                                        let sc = self.tmp_eval(&prop.value).unwrap();

                                                        match sc {
                                                            Value::Number(num) => {
                                                                circle.stretch(num, num);
                                                            }

                                                            _ => {
                                                                panic!("wrong type somewhere");
                                                            }
                                                        }
                                                        // circle.stretch(prop.value, prop.value)
                                                    }

                                                    _ => {}
                                                }
                                            }
                                        }

                                        _ => {}
                                    }

                                }
                                _ => {}
                            }
                        }

                        None => {
                            panic!("shape not found");
                        }
                    }

                    None
                },
                StatementKind::Expression(expression) => self.tmp_eval(&expression),
                StatementKind::Return(expression) => self.tmp_eval(&expression),

                StatementKind::Shift(x,y) => {
                    let x_val = self.tmp_eval(&x).unwrap();
                    let y_val = self.tmp_eval(&y).unwrap();
                    // shift the shape here // 
                    Some(Value::Tuple(vec![x_val, y_val]))
                },
                StatementKind::Stretch(x, y) => {
                    let x_val = self.tmp_eval(&x).unwrap();
                    let y_val = self.tmp_eval(&y).unwrap();
                    // stretch the shape here //
                    Some(Value::Tuple(vec![x_val, y_val]))
                },
                StatementKind::Rotate(angle) => {
                    let angle_val = self.tmp_eval(&angle);
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
                Some(Value::Number(expression.value))
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
    }
}