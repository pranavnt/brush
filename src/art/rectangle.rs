use std::ops::DerefMut;
use std::ptr::addr_of;

use crate::error::Error;
use svg::node::element::path::{Command, Data, Parameters};
use svg::node::element::tag::Path;
use svg::node::element::{Line, Path, Rectangle};
use svg::parser::Event;
use svg::Document;

use crate::art::{Drawable, Shape, BRectangle};

impl BRectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32, outline_color: Option<(u8, u8, u8)>) -> BRectangle {
        BRectangle {
            shape: Shape {
                svg: None,
                path: None,

                circ: None,
                rect: Some(Rectangle::new()
                    .set("fill", "none")
                    .set("stroke", "#000000")
                    .set("stroke-width", 1)
                    .set("width", width)
                    .set("height", height)
                    .set("x", x)
                    .set("y", y)
                ),

                center: (x, y),
                dimensions: (0.0, 0.0),
                fill: (0, 0, 0),
                outline_color: outline_color.unwrap_or((0, 0, 0)),
                outline_width: 1.0,
                rotation: 0.0,
                stretch: (1.0, 1.0),
            },

            width: width,
            height: height
        }
    }
}

impl Drawable for BRectangle {
    fn rotate(&mut self, angle: f32) {
        unimplemented!();
    }

    fn rotate_to(&mut self, angle: f32) {
        unimplemented!();
    }

    fn shift(&mut self, x: f32, y: f32) {
        self.shape.center = (self.shape.center.0 + x, self.shape.center.1 + y);
    }

    fn shift_to(&mut self, x: f32, y: f32) {
        self.shape.center = (x, y);
    }

    fn stretch(&mut self, x: f32, y: f32) {
        if x == y {
            self.radius *= x;
        }
    }

    fn stretch_to(&mut self, x: f32, y: f32) {
        if x == y {
            self.radius = x;
        }
    }

    fn hue_shift(&mut self, amount: f32) {
        self.shape.hue_shift(amount);
    }

    fn update(&mut self) {
        let o_color = format!("#{:02x?}{:02x?}{:02x?}", self.shape.outline_color.0, self.shape.outline_color.1, self.shape.outline_color.2);

        self.shape.circ = Some(Circle::new()
                    .set("fill", "none")
                    .set("stroke", o_color)
                    .set("stroke-width", 1)
                    .set("r", self.radius)
                    .set("cx", self.shape.center.0)
                    .set("cy", self.shape.center.1));
    }
}