use std::f32::consts::PI;
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
        
        self.shape.rotation += angle;
        let rad = self.shape.rotation * PI / 180.0;

        let p1 = (self.shape.center.0 - self.width / 2.0, self.shape.center.1 - self.height / 2.0);

        /*let p2 = (self.shape.center.0 + self.width / 2.0, self.shape.center.1 - self.height / 2.0);
        let p3 = (self.shape.center.0 - self.width / 2.0, self.shape.center.1 + self.height / 2.0);
        let p4 = (self.shape.center.0 + self.width / 2.0, self.shape.center.1 + self.height / 2.0);
        */
        let rp1 = (p1.0 * rad.cos() - p1.1 * rad.sin(), 
                            p1.0 * rad.sin() + p1.1 * rad.cos()); 

        /*let rp2 = (p2.0 * self.shape.rotation.cos() - p2.1 * self.shape.rotation.sin(), 
                            p2.0 * self.shape.rotation.sin() + p2.1 * self.shape.rotation.cos());

        let rp3 = (p3.0 * self.shape.rotation.cos() - p3.1 * self.shape.rotation.sin(), 
                            p3.0 * self.shape.rotation.sin() + p3.1 * self.shape.rotation.cos());

        let rp4 = (p4.0 * self.shape.rotation.cos() - p4.1 * self.shape.rotation.sin(), 
                            p4.0 * self.shape.rotation.sin() + p4.1 * self.shape.rotation.cos());
        */
        self.shape.rect = Some(Rectangle::new()
                    .set("fill", "none")
                    .set("stroke", format!("#{:02x?}{:02x?}{:02x?}", self.shape.outline_color.0, self.shape.outline_color.1, self.shape.outline_color.2))
                    .set("stroke-width", 1)
                    .set("x", rp1.0 + self.width / 2.0)
                    .set("y", rp1.1 + self.height / 2.0)
                    .set("width", self.width)
                    .set("height", self.height)
                );
        
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
        self.width *= x;
        self.height *= y;
    }

    fn stretch_to(&mut self, x: f32, y: f32) {
        self.width = x;
        self.height = y;
    }

    fn reflect(&mut self, p1: (f32, f32), p2: (f32, f32)) {
        // get line properties
        let slope = (p2.1 - p1.1) / (p2.0 - p1.0);
        let intercept = p1.1 - slope * p1.0;
        // reflect the center
        self.shape.center.0 = self.shape.center.0 - (2.0 * (slope * self.shape.center.1 - self.shape.center.0 + intercept)) / (slope.powi(2) + 1.0);
        self.shape.center.1 = self.shape.center.1 + slope * ((2.0 * (slope * self.shape.center.1 - self.shape.center.0 + intercept)) / (slope.powi(2) + 1.0))- 2.0 * intercept;
    }
    fn hue_shift(&mut self, amount: f32) {
        self.shape.hue_shift(amount);
    }

    fn update(&mut self) {
        let o_color = format!("#{:02x?}{:02x?}{:02x?}", self.shape.outline_color.0, self.shape.outline_color.1, self.shape.outline_color.2);
        /* 
        self.shape.rect = Some(Rectangle::new()
                    .set("fill", "none")
                    .set("stroke", o_color)
                    .set("stroke-width", 1)
                    .set("x", self.shape.center.0 - self.width / 2.0)
                    .set("y", self.shape.center.1 - self.height / 2.0)
                    .set("width", self.width)
                    .set("height", self.height)
                );
                */
    }

    
}