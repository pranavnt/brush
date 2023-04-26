use std::ops::DerefMut;
use std::ptr::addr_of;

use crate::error::Error;
use svg::node::element::path::{Command, Data, Parameters};
use svg::node::element::tag::Path;
use svg::node::element::{Line, Path, Circle};
use svg::parser::Event;
use svg::Document;

use crate::art::{Drawable, Shape, BCircle};

impl BCircle {
    pub fn new(x: f32, y: f32, radius: f32, outline_color: Option<(u8, u8, u8)>) -> BCircle {
        BCircle {
            shape: Shape {
                svg: None,
                path: None,

                circ: Some(Circle::new()
                    .set("fill", "none")
                    .set("stroke", "#000000")
                    .set("stroke-width", 1)
                    .set("r", radius)
                    .set("cx", x)
                    .set("cy", y)
                ),
                rect: None,

                center: (x, y),
                dimensions: (0.0, 0.0),
                fill: (0, 0, 0),
                outline_color: outline_color.unwrap_or((0, 0, 0)),
                outline_width: 1.0,
                rotation: 0.0,
                stretch: (1.0, 1.0),
            },

            radius: radius,
        }
    }
}

impl Drawable for BCircle {
    fn rotate(&mut self, angle: f32) {
        unimplemented!();
    }

    fn rotate_to(&mut self, angle: f32) {
        unimplemented!();
    }

    fn rotate_about(&mut self, angle: f32, x: f32, y: f32) {
        self.shape.rotate_about(angle, x, y);
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

    fn reflect(&mut self, p1: (f32, f32), p2: (f32, f32)) {
        self.shape.reflect(p1, p2);
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