use crate::art::{draw, BCircle, BRectangle, BPolygon, Drawable, Shape, SVG};
use crate::ast::*;
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
    Polygon(BPolygon),
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

    fn extract_numnode(node: &Node) -> f32 {
        match &*node {
            Node::NumberLiteral(num) => num.value,
            _ => panic!("wrong type somewhere"),
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
                                                    let x = Self::extract_numnode(&*x);
                                                    let y = Self::extract_numnode(&*y);
                                                    ev_shape.shift(x, y);
                                                }

                                                StatementKind::Stretch(x, y) => {
                                                    // stretch by x, y (will be same value for both lol)
                                                    let x = Self::extract_numnode(&*x);
                                                    let y = Self::extract_numnode(&*y);
                                                    ev_shape.stretch(x, y);
                                                }

                                                StatementKind::HueShift(amount) => {
                                                    let hue_offset =
                                                        Self::extract_numnode(&*amount) % 360.0;

                                                    ev_shape.hue_shift(hue_offset);
                                                }

                                                StatementKind::Rotate(angle) => {
                                                    let angle = Self::extract_numnode(&*angle);

                                                    ev_shape.rotate(angle);
                                                }

                                                StatementKind::RotateTo(angle) => {
                                                    let angle = Self::extract_numnode(&angle);

                                                    ev_shape.rotate_to(angle);
                                                }

                                                StatementKind::RotateAbout(angle, x, y) => {
                                                    let angle = Self::extract_numnode(&angle);
                                                    let x = Self::extract_numnode(&x);
                                                    let y = Self::extract_numnode(&y);

                                                    ev_shape.rotate_about(angle, x, y);
                                                }

                                                StatementKind::Reflect(p1x, p1y, p2x, p2y) => {
                                                    let p1x = Self::extract_numnode(&p1x);
                                                    let p1y = Self::extract_numnode(&*p1y);
                                                    let p2x = Self::extract_numnode(&*p2x);
                                                    let p2y = Self::extract_numnode(&*p2y);

                                                    ev_shape.reflect(p1x, p1y, p2x, p2y);
                                                }

                                                StatementKind::Warp(freq, ampl) => {
                                                    let freq = Self::extract_numnode(&*freq);
                                                    let ampl = Self::extract_numnode(&*ampl);

                                                    ev_shape.warp(freq, ampl);
                                                }
                                                _ => {
                                                    unimplemented!()
                                                }
                                            }
                                        }

                                        _ => {}
                                    }
                                }

                                ()
                            };

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
                    
                    /*
                        pos: (x, y)
                        fill: (r, g, b, a)
                        outline: (r, g, b)
                        thickness: (t)
                     */

                    let mut generic_config = (
                        (0.0, 0.0),
                        (u8::from(0), u8::from(0), u8::from(0), u8::from(0)),
                        (u8::from(0), u8::from(0), u8::from(0)),
                        (1.0)
                    );

                    let mut generations = 1;

                    // first pass to check for generic properties

                    for property in properties.clone() {
                        match property.name.as_str() {
                            "generations" => {
                                generations = Self::extract_numnode(&*property.value) as i32;
                            },

                            "position" => {
                                generic_config.0 = match *property.value {
                                    Node::TupleLiteral(tuple) => {
                                        let x = Self::extract_numnode(&*&tuple.values[0]);
                                        let y = Self::extract_numnode(&*&tuple.values[1]);

                                        (x, y)
                                    }
                                    _ => {
                                        panic!("wrong type somewhere");
                                    }
                                }
                            },

                            "fill" => {
                                generic_config.1 = match *property.value {
                                    Node::TupleLiteral(tuple) => {
                                        let r = Self::extract_numnode(&*&tuple.values[0]);
                                        let g = Self::extract_numnode(&*&tuple.values[1]);
                                        let b = Self::extract_numnode(&*&tuple.values[2]);

                                        (r as u8, g as u8, b as u8, 255)
                                    }
                                    _ => {
                                        panic!("wrong type somewhere");
                                    }
                                }
                            },

                            "outline" => {
                                generic_config.2 = match *property.value {
                                    Node::TupleLiteral(tuple) => {
                                        let r = Self::extract_numnode(&*&tuple.values[0]);
                                        let g = Self::extract_numnode(&*&tuple.values[1]);
                                        let b = Self::extract_numnode(&*&tuple.values[2]);

                                        (r as u8, g as u8, b as u8)
                                    }
                                    _ => {
                                        panic!("wrong type somewhere");
                                    }
                                }
                            },

                            "thickness" => {
                                generic_config.3 = Self::extract_numnode(&*property.value);
                            }

                            _ => ()
                        }
                    }

                    // then check shape specific properties and process boilerplate shapes
                    match shape_kind {
                        ShapeKind::Polygon => {
                            let mut x_list = Vec::<f32>::new();
                            let mut y_list = Vec::<f32>::new();

                            for property in properties {
                                match property.name.as_str() {
                                    "x" => {
                                        match *property.value {
                                            Node::TupleLiteral(tuple) => {
                                                for val in &tuple.values {
                                                    x_list.push(Self::extract_numnode(&*val));
                                                }
                                            }

                                            _ => {
                                                panic!("wrong type somewhere");
                                            }
                                        }
                                    }

                                    "y" => {
                                        match *property.value {
                                            Node::TupleLiteral(tuple) => {
                                                for val in &tuple.values {
                                                    y_list.push(Self::extract_numnode(&*val));
                                                }
                                            }

                                            _ => {
                                                panic!("wrong type somewhere");
                                            }
                                        }
                                    }

                                    _ => ()
                                }
                            }

                            // create boilerplate poly
                            let mut poly = BPolygon::new(
                                x_list,
                                y_list,
                                Some(generic_config.2),
                                generic_config.3,
                                generic_config.1
                            );

                            for i in 0..generations {
                                // push to shapes
                                poly.update();
                                self.shapes.push(poly.clone().shape);

                                poly = poly.clone();
                                evolve_fn(&mut poly, statements.clone());
                            }
                        }

                        ShapeKind::Rectangle => {
                            // size
                            let mut rect_config = (
                                (0.0, 0.0),
                            );

                            for property in properties {
                                match property.name.as_str() {
                                    "size" => {
                                        rect_config.0 = match *property.value {
                                            Node::TupleLiteral(tuple) => {
                                                let w = Self::extract_numnode(&*&tuple.values[0]);
                                                let h = Self::extract_numnode(&*&tuple.values[1]);
    
                                                (w, h)
                                            }
                                            _ => {
                                                panic!("wrong type somewhere");
                                            }
                                        }
                                    }

                                    _ => ()
                                }
                            }

                            // create boilerplate rectangle
                            let mut rect = BRectangle::new(
                                generic_config.0.0,
                                generic_config.0.1,
                                rect_config.0.0,
                                rect_config.0.1,
                                Some(generic_config.2),
                                generic_config.3,
                                generic_config.1
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
                            // radius
                            let mut circle_config = (
                                0.0,
                            );

                            // parse properties
                            for property in properties {
                                match property.name.as_str() {
                                    "radius" => {
                                        circle_config.0 = Self::extract_numnode(&*property.value);
                                    },

                                    _ => ()
                                }
                            }

                            // create boilerplate circle with radius and center and thickness

                            let mut circle = BCircle::new(
                                generic_config.0.0,
                                generic_config.0.1,
                                circle_config.0,
                                Some(generic_config.2),
                                generic_config.3,
                                generic_config.1
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
