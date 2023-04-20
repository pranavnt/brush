use std::ops::DerefMut;
use std::ptr::addr_of;

use crate::error::Error;
use svg::node::element::path::{Command, Data, Parameters};
use svg::node::element::tag::Path;
use svg::node::element::{Line, Path, Polygon};
use svg::parser::Event;
use svg::Document;

use crate::art::{Drawable, Shape, BPolygon};

impl BPolygon {
    pub fn new(pts: Vec<(f32, f32)>, outline_color: Option<(u8, u8, u8)>) -> BPolygon {
        if pts.len() < 3 {
            panic!("polygon can't have less than 3 vertices");
        }

        let mut cdata = Data::new();
        cdata = cdata.move_to((pts[0].0, pts[0].1));

        let mut sumx = pts[0].0;
        let mut sumy = pts[0].1;

        for (tx, ty) in pts.iter().skip(0) {
            sumx += tx;
            sumy += ty;
            cdata = cdata.line_to((*tx, *ty));
        }

        cdata = cdata.close();

        BPolygon {
            shape: Shape {
                svg: Some(Path::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 1)
                    .set("d", cdata.clone())),

                path: Some(cdata.clone()),

                circ: None,
                rect: None,

                center: (sumx / pts.len() as f32, sumy / pts.len() as f32),
                dimensions: (0.0, 0.0),
                fill: (0, 0, 0),
                outline_color: outline_color.unwrap_or((0, 0, 0)),
                outline_width: 1.0,
                rotation: 0.0,
                stretch: (1.0, 1.0),
            },

            points: pts.clone()
        }
    }
}

impl Drawable for BPolygon {
    fn rotate(&mut self, angle: f32) {
        unimplemented!();
    }

    fn rotate_to(&mut self, angle: f32) {
        unimplemented!();
    }

    fn shift(&mut self, x: f32, y: f32) {
        self.shape.shift(x, y);
    }

    fn shift_to(&mut self, x: f32, y: f32) {
        unimplemented!();
    }

    fn stretch(&mut self, x: f32, y: f32) {
        unimplemented!();
    }

    fn stretch_to(&mut self, x: f32, y: f32) {
        unimplemented!();
    }

    fn hue_shift(&mut self, amount: f32) {
        self.shape.hue_shift(amount);
    }

    fn update(&mut self) {
        self.shape.update();
    }
}