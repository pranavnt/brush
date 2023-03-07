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
    dimensions: (f64, f64),
    fill: (u8, u8, u8),
    outline_color: (u8, u8, u8),
    outline_width: f64,
    rotation: f64,
    stretch: (f64, f64),
    warp: f64,
}

impl Shape {
    pub fn new_circle(
        center: (f64, f64),
        radius: f64,
    ) -> Shape {  
        let mut path = Data::new();
        let mut x = center.0 + radius;
        let mut y = center.1;
        let mut angle: f32 = 0.0;
        while angle < 360.0 {
            path = path.add(x, y);
            x = center.0 + radius * angle.cos();
            y = center.1 + radius * angle.sin();
            angle += 1.0;
        }

        unimplemented!();

    }

    pub fn new_rect() -> Shape {
        unimplemented!();
    }

    pub fn new_triangle() -> Shape {
        unimplemented!();
    }

    pub fn new_polygon() -> Shape {
        unimplemented!();
    }

    pub fn shift(&mut self, x: f64, y: f64) {
        self.center.0 += x;
        self.center.1 += y;

        // iterate through the path and shift each point   
    }

    pub fn rotate(&mut self, angle: f64) {
        self.rotation += angle;

        // iterate through the path and rotate each point around the center
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