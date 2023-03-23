use svg::Document;
use svg::parser::Event;
use svg::node::element::Path;
use svg::node::element::tag::Path;
use svg::node::element::path::{Command, Data};
use crate::error::Error;

pub struct Shape {
    pub svg: Path,
    path: Data,
    center: (f64, f64),
    fill: (u8, u8, u8),
    outline_color: (u8, u8, u8),
    outline_width: f64,
    rotation: f64,
    stretch: (f64, f64),
}

trait Drawable {
    fn rotate(&mut self, angle: f64);
    fn rotate_to(&mut self, angle: f64);
    fn shift(&mut self, x: f64, y: f64);
    fn shift_to(&mut self, x: f64, y: f64);
    fn stretch(&mut self, x: f64, y: f64);
    fn stretch_to(&mut self, x: f64, y: f64);
    fn update(&mut self);
}

pub struct Circle {
    shape: Shape,
    radius: f64,
}

pub struct Rectangle {
    shape: Shape,
    width: f64,
    height: f64,
}

pub struct Polygon {
    shape: Shape,
    points: Vec<(f64, f64)>,
}

pub struct SVG {
    shape: Shape,
    dimensions: (f64, f64),
}

impl Circle {
    pub fn new() -> Circle {
        Circle {
            shape: Shape {
                svg: Path::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 1),
                path: Data::new(),
                center: (0.0, 0.0),
                fill: (0, 0, 0),
                outline_color: (0, 0, 0),
                outline_width: 1.0,
                rotation: 0.0,
                stretch: (1.0, 1.0),
            },
            radius: 0.0,
        }
    }

    pub fn new_default() -> Circle {
        Circle {
            shape: Shape {
                svg: Path::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 1),
                path: Data::new(),
                center: (0.0, 0.0),
                fill: (0, 0, 0),
                outline_color: (0, 0, 0),
                outline_width: 1.0,
                rotation: 0.0,
                stretch: (1.0, 1.0),
            },
            radius: 0.0,
        }
    }
}

impl Drawable for Circle {
    fn rotate(&mut self, angle: f64) {
        unimplemented!();
    }

    fn rotate_to(&mut self, angle: f64) {
        unimplemented!();
    }

    fn shift(&mut self, x: f64, y: f64) {
        unimplemented!();
    }

    fn shift_to(&mut self, x: f64, y: f64) {
        unimplemented!();
    }

    fn stretch(&mut self, x: f64, y: f64) {
        unimplemented!();
    }

    fn stretch_to(&mut self, x: f64, y: f64) {
        unimplemented!();
    }
     
    fn update(&mut self) {
        unimplemented!();
    }
}


pub fn draw(shapes: Vec::<Shape>) -> Result<(), Error> { 
    let mut canvas: Document = Document::new()
        .set("viewBox", (0, 0, 1000, 1000))
        .set("width", "100%")
        .set("height", "100%")
        .set("preserveAspectRatio", "xMidYMid meet");
    
    for shape in shapes {
        canvas = canvas.add(shape.svg);
    }

    svg::save("art.svg", &canvas).unwrap();

    Ok(())
}
