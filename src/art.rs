use svg::Document;
use svg::parser::Event;
use svg::node::element::Path;
use svg::node::element::tag::Path;
use svg::node::element::path::{Command, Data};
use error::Error::*;

struct Shape {
    svg: Path,
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
    fn new(
        shape: &str,
        center: (f64, f64),
        dimensions: (f64, f64),
        fill: (u8, u8, u8),
        outline_color: (u8, u8, u8),
        outline_width: f64,
        rotation: f64,
        stretch: (f64, f64),
        warp: f64,
    ) -> Shape {
        match shape {
            "circle" => {
                unimplemented!();
            },
            "rectangle" => {
                unimplemented!();
            },
            "triangle" => {
                unimplemented!();
            },
            "polygon" => {
                unimplemented!();
            },
            _ => {
                // throw error
                throw ;
            }
        }
    }
}