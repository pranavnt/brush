use crate::ast::*;
use crate::art::{Shape, BCircle, BRectangle, Polygon, SVG, Drawable, draw};
use crate::tokens::{Token, TokenType};
use std::collections::*;

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
    Evolve(ShapeKind, EvolveFn, Vec<Node>),
    Shape(Shapes),
    Statements(Vec<Node>),
}

pub type EvolveFn = for<'a> fn(&'a mut dyn Drawable, Vec<Node>) -> ();

#[derive(Debug, Clone)]
pub enum Shapes {
    Circle(BCircle),
    Rectangle(BRectangle),
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
        for statement in self.ast.statements.clone().iter() {
            self.eval(statement.clone());
        }
    }

    pub fn eval(&mut self, node: Node) -> Option<Value> {
        match node {
            Node::Program(program) => {
                // this code is never going to reach ???
                None
            }

            Node::Shape(shape) => {
                let name = &shape.name;
                match self.symbol_table.get(name) {
                    Some(value) => panic!("variable already declared with name: {}", name),
                    None => {
                        let evolve_fn: for<'a> fn(&'a mut dyn Drawable, Vec<_>) -> _ =
                            |ev_shape: &mut dyn Drawable, statements: Vec<Node>| {
                                for statement in statements {
                                    match statement {
                                        Node::Statement(statement) => {
                                            match statement.kind {
                                                StatementKind::Shift(x, y) => {
                                                    // shift by x, y
                                                    let x = match *x {
                                                        Node::NumberLiteral(num) => {
                                                            num.value
                                                        }
                                                        _ => {
                                                            panic!("wrong type somewhere");
                                                        }
                                                    };
                                                    let y = match *y {
                                                        Node::NumberLiteral(num) => {
                                                            num.value
                                                        }
                                                        _ => {
                                                            panic!("wrong type somewhere");
                                                        }
                                                    };
                                                    ev_shape.shift(x, y);
                                                }

                                                StatementKind::Stretch(x, y) => {
                                                    // stretch by x, y (will be same value for both lol)
                                                    let x = match *x {
                                                        Node::NumberLiteral(num) => {
                                                            num.value
                                                        }
                                                        _ => {
                                                            panic!("wrong type somewhere");
                                                        }
                                                    };
                                                    let y = match *y {
                                                        Node::NumberLiteral(num) => {
                                                            num.value
                                                        }
                                                        _ => {
                                                            panic!("wrong type somewhere");
                                                        }
                                                    };
                                                    ev_shape.stretch(x, y);
                                                }

                                                StatementKind::HueShift(amount) => {
                                                    let hue_offset = match *amount {
                                                        Node::NumberLiteral(num) => {
                                                            // mod by 360 degrees protects shift amount
                                                            num.value % 360.0
                                                        }

                                                        _ => {
                                                            panic!("wrong type somewhere");
                                                        }
                                                    };

                                                    ev_shape.hue_shift(hue_offset);
                                                }

                                                StatementKind::Rotate(angle) => {
                                                    let angle = match *angle {
                                                        Node::NumberLiteral(num) => {
                                                            num.value
                                                        }
                                                        _ => {
                                                            panic!("wrong type somewhere");
                                                        }
                                                    };

                                                    ev_shape.rotate(angle);
                                                }

                                                StatementKind::RotateTo(angle) => {
                                                    let angle = match *angle {
                                                        Node::NumberLiteral(num) => {
                                                            num.value
                                                        }
                                                        _ => {
                                                            panic!("wrong type somewhere");
                                                        }
                                                    };

                                                    ev_shape.rotate(angle);
                                                }

                                                StatementKind::RotateAbout(angle, x, y) => {
                                                    let angle = match *angle {
                                                        Node::NumberLiteral(num) => {
                                                            num.value
                                                        }
                                                        _ => {
                                                            panic!("wrong type somewhere");
                                                        }
                                                    };

                                                    let x = match *x {
                                                        Node::NumberLiteral(num) => {
                                                            num.value
                                                        }
                                                        _ => {
                                                            panic!("wrong type somewhere");
                                                        }
                                                    };

                                                    let y = match *y {
                                                        Node::NumberLiteral(num) => {
                                                            num.value
                                                        }

                                                        _ => {
                                                            panic!("wrong type somewhere");
                                                        }
                                                    };

                                                    ev_shape.rotate_about(angle, x, y);
                                                }

                                                StatementKind::Reflect(p1x, p1y, p2x, p2y) => {
                                                    let p1x = match *p1x {
                                                        Node::NumberLiteral(num) => {
                                                            num.value
                                                        }
                                                        _ => {
                                                            panic!("wrong type somewhere");
                                                        }
                                                    };

                                                    let p1y = match *p1y {
                                                        Node::NumberLiteral(num) => {
                                                            num.value
                                                        }
                                                        _ => {
                                                            panic!("wrong type somewhere");
                                                        }
                                                    };

                                                    let p2x = match *p2x {
                                                        Node::NumberLiteral(num) => {
                                                            num.value
                                                        }
                                                        _ => {
                                                            panic!("wrong type somewhere");
                                                        }
                                                    };

                                                    let p2y = match *p2y {
                                                        Node::NumberLiteral(num) => {
                                                            num.value
                                                        }

                                                        _ => {
                                                            panic!("wrong type somewhere");
                                                        }
                                                    };

                                                    ev_shape.reflect(p1x, p1y, p2x, p2y);
                                                }

                                                StatementKind::Warp(freq, ampl) => {
                                                    let freq = match *freq {
                                                        Node::NumberLiteral(num) => {
                                                            num.value
                                                        }

                                                        _ => {
                                                            panic!("wrong type somewhere");
                                                        }
                                                    };

                                                    let ampl = match *ampl {
                                                        Node::NumberLiteral(num) => {
                                                            num.value
                                                        }

                                                        _ => {
                                                            panic!("wrong type somewhere");
                                                        }
                                                    };
                                                    ev_shape.warp(freq, ampl);
                                                }
                                                _ => {  unimplemented!() }
                                            }
                                        }

                                        _ => {}
                                    }
                                }

                                ()
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

                        self.symbol_table.insert(
                            name.clone(),
                            Value::Evolve(shape.kind, evolve_fn, shape.statements.clone()),
                        );
                    }
                }

                None
            }

            Node::Statement(statement) => match statement.clone().kind {
                StatementKind::DrawShape(name, properties) => {
                    // println!("{:#?}\n\n", statement);

                    let (shape_kind, evolve_fn, statements) = match self.symbol_table.get(&name) {
                        Some(value) => match value {
                            Value::Evolve(shapeKind, evolveFn, statements) => {
                                (shapeKind, evolveFn, statements)
                            }
                            _ => {
                                panic!("wrong type somewhere");
                            }
                        },
                        _ => {
                            panic!("shape not found");
                        }
                    };

                    match shape_kind {
                        ShapeKind::Rectangle => {
                            // ((x,y), (w, h), color)
                            let mut rect_config = (
                                (0.0, 0.0),
                                (0.0, 0.0),
                                (u8::from(0), u8::from(0), u8::from(0)),
                            );
                            let mut generations = 1;

                            for property in properties {
                                if property.name == "position" {
                                    rect_config.0 = match *property.value {
                                        Node::TupleLiteral(tuple) => {
                                            // idk fix this by adding more nested matches
                                            // and whatever is below
                                            let x = match &tuple.values[0] {
                                                Node::NumberLiteral(num) => num.value,
                                                _ => {
                                                    panic!("wrong type somewhere");
                                                }
                                            };

                                            let y = match &tuple.values[1] {
                                                Node::NumberLiteral(num) => num.value,
                                                _ => {
                                                    panic!("wrong type somewhere");
                                                }
                                            };

                                            (x, y)
                                        }
                                        _ => {
                                            panic!("wrong type somewhere");
                                        }
                                    }
                                } else if property.name == "size" {
                                    rect_config.1 = match *property.value {
                                        Node::TupleLiteral(tuple) => {
                                            // idk fix this by adding more nested matches
                                            // and whatever is below
                                            let w = match &tuple.values[0] {
                                                Node::NumberLiteral(num) => num.value,
                                                _ => {
                                                    panic!("wrong type somewhere");
                                                }
                                            };

                                            let h = match &tuple.values[1] {
                                                Node::NumberLiteral(num) => num.value,
                                                _ => {
                                                    panic!("wrong type somewhere");
                                                }
                                            };

                                            (w, h)
                                        }
                                        _ => {
                                            panic!("wrong type somewhere");
                                        }
                                    }
                                } else if property.name == "generations" {
                                    generations = match *property.value {
                                        Node::NumberLiteral(num) => num.value as i32,
                                        _ => {
                                            panic!("wrong type somewhere");
                                        }
                                    }
                                } else if property.name == "color" {
                                    rect_config.2 = match *property.value {
                                        Node::TupleLiteral(tuple) => {
                                            // idk fix this by adding more nested matches
                                            // and whatever is below
                                            let r = match &tuple.values[0] {
                                                Node::NumberLiteral(num) => num.value,
                                                _ => {
                                                    panic!("wrong type somewhere");
                                                }
                                            };

                                            let g = match &tuple.values[1] {
                                                Node::NumberLiteral(num) => num.value,
                                                _ => {
                                                    panic!("wrong type somewhere");
                                                }
                                            };

                                            let b = match &tuple.values[2] {
                                                Node::NumberLiteral(num) => num.value,
                                                _ => {
                                                    panic!("wrong type somewhere");
                                                }
                                            };

                                            (r as u8, g as u8, b as u8)
                                        }
                                        _ => {
                                            panic!("wrong type somewhere");
                                        }
                                    }
                                }
                            }

                            // create boilerplate rectangle
                            let mut rect = BRectangle::new(
                                rect_config.0 .0,
                                rect_config.0 .1,
                                rect_config.1 .0,
                                rect_config.1 .1,
                                Some(rect_config.2),
                            );

                            for i in 0..generations {
                                // push to shapes
                                rect.update();
                                self.shapes.push(rect.clone().shape);

                                rect = rect.clone();
                                evolve_fn(&mut rect, statements.clone());
                            }
                        } 

                        ShapeKind::Circle => {
                            // (radius, center, color)
                            let mut circle_config = (0.0, (0.0, 0.0), (u8::from(0), u8::from(0), u8::from(0)), 0.0);
                            let mut generations = 1;

                            // parse properties
                            for property in properties {
                                if property.name == "radius" {
                                    circle_config.0 = match *property.value {
                                        Node::NumberLiteral(num) => num.value,
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
                                                Node::NumberLiteral(num) => num.value,
                                                _ => {
                                                    panic!("wrong type somewhere");
                                                }
                                            };

                                            let y = match &tuple.values[1] {
                                                Node::NumberLiteral(num) => num.value,
                                                _ => {
                                                    panic!("wrong type somewhere");
                                                }
                                            };

                                            (x, y)
                                        }
                                        _ => {
                                            panic!("wrong type somewhere");
                                        }
                                    }
                                } else if property.name == "generations" {
                                    generations = match *property.value {
                                        Node::NumberLiteral(num) => num.value as i32,
                                        _ => {
                                            panic!("wrong type somewhere");
                                        }
                                    }
                                } else if property.name == "color" {
                                    circle_config.2 = match *property.value {
                                        Node::TupleLiteral(tuple) => {
                                            // idk fix this by adding more nested matches
                                            // and whatever is below
                                            let r = match &tuple.values[0] {
                                                Node::NumberLiteral(num) => num.value,
                                                _ => {
                                                    panic!("wrong type somewhere");
                                                }
                                            };

                                            let g = match &tuple.values[1] {
                                                Node::NumberLiteral(num) => num.value,
                                                _ => {
                                                    panic!("wrong type somewhere");
                                                }
                                            };

                                            let b = match &tuple.values[2] {
                                                Node::NumberLiteral(num) => num.value,
                                                _ => {
                                                    panic!("wrong type somewhere");
                                                }
                                            };

                                            (r as u8, g as u8, b as u8)
                                        }
                                        _ => {
                                            panic!("wrong type somewhere");
                                        }
                                    }
                                } else if property.name=="thickness"{
                                    circle_config.3 = match *property.value{
                                        Node::NumberLiteral(num) =>{
                                            num.value
                                        },
                                        _=>{
                                            panic!("wrong type");
                                        }
                                    }
        
                                } else {
                                    panic!("unknown property");
                                }
                            }

                            // create boilerplate circle with radius and center and thickness 
                            
                            let mut th = circle_config.3;
                            if (circle_config.3==0.0){
                                th = 1.0;
                        }

                        let mut circle = BCircle::new(
                            circle_config.1.0,
                            circle_config.1.1,
                            circle_config.0,
                            Some(circle_config.2), 
                            th
                        );

                        for i in 0..generations {
                            // push to shapes
                            circle.update();
                            self.shapes.push(circle.clone().shape);

                            circle = circle.clone();
                            // circle.hue_shift(5.0);
                            evolve_fn(&mut circle, statements.clone());
                        }

                        }

                        _ => {
                            unimplemented!()
                        }
                    }

                    match draw(self.shapes.clone()) {
                        Ok(_) => {}
                        Err(e) => {
                            panic!("error drawing");
                        }
                    }

                    None
                }
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
            }

            Node::BinaryExpression(expression) => {
                unimplemented!()
            }
            Node::Block(expression) => {
                unimplemented!()
            }
            Node::Property(expression) => {
                unimplemented!()
            }
            Node::NumberLiteral(expression) => Some(Value::Number(expression.value)),
            Node::StringLiteral(expression) => {
                unimplemented!()
            }
            Node::BooleanLiteral(expression) => {
                unimplemented!()
            }
            Node::TupleLiteral(expression) => {
                // this needs to be fixed to handle tuples > 2 things
                let mut tup: Vec<Value> = Vec::new();

                for val in &expression.values {
                    tup.push(self.eval(val.clone()).unwrap());
                }

                Some(Value::Tuple(tup))
            }
            Node::FunctionCall(expression) => {
                unimplemented!()
            }
            Node::VariableDeclaration(expression) => {
                unimplemented!()
            }
            Node::VariableAssignment(expression) => {
                unimplemented!()
            }
            Node::IfStatement(expression) => {
                unimplemented!()
            }
            Node::WhileLoop(expression) => {
                unimplemented!()
            }
            Node::ForLoop(expression) => {
                unimplemented!()
            }
        }
    }
}
