use std::collections::*;
use crate::ast::*;
use crate::art::{Circle, Rectangle, Polygon, SVG, Shape, Drawable, draw};
use crate::tokens::{Token, TokenType};

pub struct Interpreter {
    pub ast: ProgramNode,
    pub symbol_table: HashMap<String, Value>,
    pub call_stack: Vec<Node>,
    pub shapes: Vec<Shape>,
}

pub enum Value {
    Number(f32),
    String(String),
    Tuple(Vec<Value>),
    Evolve(EvolveFn, Vec<Node>),
    Shape(Shapes),
    Statements(Vec<Node>),
}

pub type EvolveFn = for<'a> fn(&'a mut Circle, Vec<Node>) -> ();

#[derive(Debug, Clone)]
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
            self.eval(statement.clone());
        }


    }

    pub fn eval(&mut self, node: Node) -> Option<Value> {
        match node {
            Node::Program(program) => { // this code is never going to reach ???
                None
            }

            Node::Shape(shape) => {
                let name = &shape.name;
                match self.symbol_table.get(name) {
                    Some(value) => panic!("variable already declared with name: {}", name),
                    None => {
                        let evolve_fn: for<'a> fn(&'a mut Circle, Vec<_>) -> _ = |circle: &mut Circle, statements: Vec<Node>| {
                            // Access variables from above and modify the Circle struct
                            for statement in statements {
                                match statement {
                                    Node::Statement(statement) => {
                                        match statement.kind {
                                            StatementKind::Shift(x, y) => {
                                                // shift by x, y
                                                let x = match *x {
                                                    Node::NumberLiteral(num) => {
                                                        num.value
                                                    },
                                                    _ => {
                                                        panic!("wrong type somewhere");
                                                    }
                                                };
                                                let y = match *y {
                                                    Node::NumberLiteral(num) => {
                                                        num.value
                                                    },
                                                    _ => {
                                                        panic!("wrong type somewhere");
                                                    }
                                                };
                                                circle.shift(x, y);
                                            }

                                            StatementKind::Stretch(x, y) => {
                                                // stretch by x, y (will be same value for both lol) 
                                                let x = match *x {
                                                    Node::NumberLiteral(num) => {
                                                        num.value
                                                    },
                                                    _ => {
                                                        panic!("wrong type somewhere");
                                                    }
                                                };
                                                let y = match *y {
                                                    Node::NumberLiteral(num) => {
                                                        num.value
                                                    },
                                                    _ => {
                                                        panic!("wrong type somewhere");
                                                    }
                                                };
                                                circle.stretch(x, y);
                                            }

                                            _ => {}
                                        }
                                    }

                                    _ => {}
                                }
                            }
                        };                        
                        // let evolveFn = |circle: &Circle| {
                        //     for statement in &shape.statements {
                        //         // if statement is shift, then shift by the value
                        //         // if statement is stretch, then stretch by the value

                        //         match statement {
                        //             Node::Statement(statement) => {
                        //                 match statement.kind {
                        //                     StatementKind::Shift(x, y) => {
                        //                         // shift by x, y
                        //                         let x = match *x {
                        //                             Node::NumberLiteral(num) => {
                        //                                 num.value
                        //                             },
                        //                             _ => {
                        //                                 panic!("wrong type somewhere");
                        //                             }
                        //                         };
                        //                         let y = match *y {
                        //                             Node::NumberLiteral(num) => {
                        //                                 num.value
                        //                             },
                        //                             _ => {
                        //                                 panic!("wrong type somewhere");
                        //                             }
                        //                         };
                        //                         circle.shape.shift(x, y);
                        //                     }

                        //                     StatementKind::Stretch(x, y) => {
                        //                         // stretch by x, y (will be same value for both lol) 
                        //                         let x = match *x {
                        //                             Node::NumberLiteral(num) => {
                        //                                 num.value
                        //                             },
                        //                             _ => {
                        //                                 panic!("wrong type somewhere");
                        //                             }
                        //                         };
                        //                         let y = match *y {
                        //                             Node::NumberLiteral(num) => {
                        //                                 num.value
                        //                             },
                        //                             _ => {
                        //                                 panic!("wrong type somewhere");
                        //                             }
                        //                         };
                        //                         circle.shape.stretch(x, y);
                        //                     }

                        //                     _ => {}
                        //                 }
                        //             }

                        //             _ => {}
                        //         }
                        //     }
                        // };

                        self.symbol_table.insert(name.clone(), Value::Evolve(evolve_fn, shape.statements.clone()));
                    }
                }

                None
            },

            Node::Statement(statement) => match statement.clone().kind {
                StatementKind::DrawShape(name, properties) => {
                    // println!("{:#?}\n\n", statement);

                    let (evolve_fn, statements) = match self.symbol_table.get(&name) {
                        Some(value) => {
                            match value {
                                Value::Evolve(evolveFn, statements) => {
                                    (evolveFn, statements)
                                },
                                _ => {
                                    panic!("wrong type somewhere");
                                }
                            }
                        },
                        _ => {
                            panic!("shape not found");
                        }
                    };

                    let mut circle_config = (0.0, (0.0, 0.0));
                    let mut generations = 1;

                    // parse properties
                    for property in properties {
                        if property.name == "radius" {
                            circle_config.0 = match *property.value {
                                Node::NumberLiteral(num) => {
                                    num.value
                                },
                                _ => {
                                    panic!("wrong type somewhere");
                                }
                            }
                        } else if property.name == "center" {
                            circle_config.1 = match *property.value {
                                Node::TupleLiteral(tuple) => {
                                    // idk fix this by adding more nested matches 
                                    // and whatever is below
                                    let x = match &tuple.values[0] {
                                        Node::NumberLiteral(num) => {
                                            num.value
                                        },
                                        _ => {
                                            panic!("wrong type somewhere");
                                        }
                                    };

                                    let y = match &tuple.values[1] {
                                        Node::NumberLiteral(num) => {
                                            num.value
                                        },
                                        _ => {
                                            panic!("wrong type somewhere");
                                        }
                                    };

                                    (x, y)
                                },
                                _ => {
                                    panic!("wrong type somewhere");
                                }
                            }
                        } else if property.name == "generations" {
                            generations = match *property.value {
                                Node::NumberLiteral(num) => {
                                    num.value as i32
                                },
                                _ => {
                                    panic!("wrong type somewhere");
                                }
                            }
                        } else {
                            panic!("unknown property");
                        }
                    }

                    // create boilerplate circle with radius and center
                    let mut circle = Circle::new(
                        circle_config.1.0,
                        circle_config.1.1,
                        circle_config.0,
                    );

                    for i in 0..generations {
                        // push to shapes
                        self.shapes.push(circle.clone().shape);

                        circle = circle.clone();
                        evolve_fn(&mut circle, statements.clone());
                    }

                    match draw(self.shapes.clone()) {
                        Ok(_) => {},
                        Err(e) => {
                            panic!("error drawing");
                        }
                    }

                    None
                },
                // StatementKind::Expression(expression) => self.tmp_eval(*expression),
                // StatementKind::Return(expression) => self.tmp_eval(*expression),

                // StatementKind::Shift(x,y) => {
                //     let x_val = self.tmp_eval(*x).unwrap();
                //     let y_val = self.tmp_eval(*y).unwrap();
                //     // shift the shape here // 
                //     // Some(Value::Tuple(vec![x_val, y_val]))

                //     None
                // },
                // StatementKind::Stretch(x, y) => {
                //     let x_val = self.tmp_eval(*x).unwrap();
                //     let y_val = self.tmp_eval(*y).unwrap();
                //     // stretch the shape here //
                //     // Some(Value::Tuple(vec![x_val, y_val]))

                //     None
                // },
                // StatementKind::Rotate(angle) => {
                //     let angle_val = self.tmp_eval(*angle);
                //     // rotate the shape here //
                //     angle_val
                // },
                _ => None,
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
            Node::TupleLiteral(expression) => { // this needs to be fixed to handle tuples > 2 things
                let mut tup: Vec<Value> = Vec::new();

                for val in &expression.values {
                    tup.push(
                        self.eval(val.clone()).unwrap()
                    );
                }

                Some(Value::Tuple(tup))
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