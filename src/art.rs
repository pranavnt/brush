use std::ops::DerefMut;
use std::ptr::addr_of;

use svg::Document;
use svg::parser::Event;
use svg::node::element::{Path, Line};
use svg::node::element::tag::Path;
use svg::node::element::path::{Command, Data, Parameters};
use crate::error::Error;

pub struct Shape {
    pub svg: Path,
    path: Data,
    center: (f32, f32),
    dimensions: (f32, f32),
    fill: (u8, u8, u8),
    outline_color: (u8, u8, u8),
    outline_width: f32,
    rotation: f32,
    stretch: (f32, f32),
    warp: f32,
}

impl Shape {
    pub fn new_circle(
        center: (f32, f32),
        radius: f32,
    ) -> Shape {  
        let mut path = Data::new();
        let mut x = center.0 + radius;
        let mut y = center.1;
        let mut angle = 0.0;

        while angle < 360.0 {
            // path.move_to defines the starting point of the path
            // path.line_to defines 
            // path = path.move_to((x, y));
            // let delta_x = radius * angle.cos();
            // let delta_y = radius * angle.sin();
            // path.line_to((center.0 + delta_x, center.1 + delta_y));
            // x = center.0 + delta_x;
            // y = center.1 + delta_y;
            angle += 1.0;
        }

        let svg = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("d", path);
        
        // Shape {
        //     svg: svg,
        //     path: path,
        //     center: center,
        //     dimensions: (radius * 2.0, radius * 2.0),
        // }
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

    pub fn shift(&mut self, x: f32, y: f32) {
        self.center.0 += x;
        self.center.1 += y;

        // iterate through the path and shift each point
        let mut cdata = self.path.clone();
        let mut newData = Data::new();
        
        // bruh we have to handle each type of command
        for cmd in cdata.iter() { // derefererence error here
            match cmd {
                Command::Move(pos, para) => {
                    newData = newData.move_to((para.get(0).unwrap() + x, para.get(1).unwrap() + y));
                }

                Command::Line(pos, para) => {
                    newData = newData.move_to((para.get(0).unwrap() + x, para.get(1).unwrap() + y));
                }

                Command::HorizontalLine(pos, para) => {
                    unimplemented!();
                }

                Command::VerticalLine(pos, para) => {
                    unimplemented!();
                }

                Command::QuadraticCurve(pos, para) => {
                    unimplemented!();
                }

                Command::SmoothQuadraticCurve(pos, para) => {
                    unimplemented!();
                }

                Command::SmoothCubicCurve(pos, para) => {
                    unimplemented!();
                }

                Command::EllipticalArc(pos, para) => {
                    unimplemented!();
                }

                Command::CubicCurve(pos, para) => {
                    unimplemented!();
                }

                Command::Close => {}
            }
        }

        self.path = newData.close();

        // need to recompute svg Path based on transformed Data

        // self.svg = Path::new()
        // .set("fill", "none")
        // .set("stroke", "black")
        // .set("stroke-width", 1)
        // .set("d", self.path);

    }

    pub fn rotate(&mut self, angle: f32) {
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