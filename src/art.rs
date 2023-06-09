mod circle;
mod rectangle;
mod shape;
mod polygon;

use std::ops::DerefMut;
use std::ptr::addr_of;

use crate::error::Error;
use svg::node::element::path::{Command, Data, Parameters};
use svg::node::element::tag::Path;
use svg::node::element::{Line, Path, Circle, Rectangle, Polygon};
use svg::parser::Event;
use svg::Document;

// use crate::eval::art::circle::{};

pub trait Drawable {
    fn rotate(&mut self, angle: f32);
    fn rotate_to(&mut self, angle: f32);
    fn rotate_about(&mut self, angle: f32, x: f32, y: f32);
    fn shift(&mut self, x: f32, y: f32);
    fn shift_to(&mut self, x: f32, y: f32);
    fn stretch(&mut self, x: f32, y: f32);
    fn stretch_to(&mut self, x: f32, y: f32);
    fn reflect(&mut self, p1x: f32, p1y: f32, p2x: f32, p2y: f32);
    fn warp(&mut self, freq: f32, ampl: f32);
    fn hue_shift(&mut self, amount: f32);
    fn update(&mut self);
}

#[derive(Debug, Clone)]
pub struct Shape {
    // for custom shapes
    pub svg: Option<Path>,
    pub path: Option<Data>,

    // for presets
    pub circ: Option<Circle>,
    pub rect: Option<Rectangle>,

    pub center: (f32, f32),
    pub dimensions: (f32, f32),
    pub fill: (u8, u8, u8, u8),
    pub outline_color: (u8, u8, u8),
    pub outline_width: f32,
    pub rotation: f32,
    pub transformation_stack: String,
    pub warp_vals: (f32, f32),
    pub stretch: (f32, f32),
}

#[derive(Debug, Clone)]
pub struct BCircle {
    pub shape: Shape,
    pub radius: f32,
}

#[derive(Debug, Clone)]
pub struct BRectangle {
    pub shape: Shape,
    pub width: f32,
    pub height: f32,

}

#[derive(Debug, Clone)]
pub struct BPolygon {
    pub shape: Shape,
    pub points: Vec<(f32, f32)>,
}

#[derive(Debug, Clone)]
pub struct SVG {
    shape: Shape,
    dimensions: (f32, f32),
}
static mut SVG_STRING: Option<String> = None;

pub fn draw(shapes: Vec<Shape>) -> Result<String, Error> {
    let mut canvas: Document = Document::new()
        .set("viewBox", (0, 0, 1000, 1000))
        .set("width", "100%")
        .set("height", "100%")
        .set("preserveAspectRatio", "xMidYMid meet");

    for shape in shapes {
        if shape.circ.is_some() {
            canvas = canvas.add(shape.circ.unwrap());
        }
        else if shape.rect.is_some() {
            canvas = canvas.add(shape.rect.unwrap());
        }
        else {
            canvas = canvas.add(shape.svg.unwrap());
        }
    }
    unsafe {
        SVG_STRING = Some(canvas.to_string());
    }
    Ok(canvas.to_string())

}   

pub fn name() -> String {
    unsafe {
        match &SVG_STRING {
            Some(s) => String::from(s.clone()),
            None => String::new(),
        }
    }
}