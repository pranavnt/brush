use std::ops::DerefMut;
use std::ptr::addr_of;

use crate::error::Error;
use svg::node::element::path::{Command, Data, Parameters};
use svg::node::element::tag::Path;
use svg::node::element::{Line, Path, Circle};
use svg::parser::Event;
use svg::Document;

use crate::art::{Drawable, Shape};

impl Drawable for Shape {
    
    fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
        self.point_of_rotation = self.center;
        
    }

    fn rotate_to(&mut self, angle: f32) {
        self.rotation = angle;
        self.point_of_rotation = self.center;
    }

    fn rotate_about(&mut self, angle: f32, x: f32, y: f32) {
        self.rotation += angle;
        self.point_of_rotation = (x, y);
    }
    
    fn shift(&mut self, x: f32, y: f32) {
        self.center.0 += x;
        self.center.1 += y;

        // iterate through the path and shift each point
        let mut cdata = self.path.clone().unwrap();
        let mut newData = Data::new();

        // bruh we have to handle each type of command
        for cmd in cdata.iter() {
            // derefererence error here
            match cmd {
                Command::Move(_pos, para) => {
                    newData = newData.move_to((para.get(0).unwrap() + x, para.get(1).unwrap() + y));
                }

                Command::Line(_pos, para) => {
                    newData = newData.line_to((para.get(0).unwrap() + x, para.get(1).unwrap() + y));
                }

                Command::Close => {}

                _ => {  unimplemented!() }
            }
        }

        self.path = Some(newData.close());
    }

    fn shift_to(&mut self, x: f32, y: f32) {
        unimplemented!();
    }

    fn stretch(&mut self, x: f32, y: f32) {
        self.center.0 *= x;
        self.center.1 *= y;

        let mut cdata = self.path.clone().unwrap();
        let mut newData = Data::new();

        // bruh we have to handle each type of command
        for cmd in cdata.iter() {
            // derefererence error here
            match cmd {
                Command::Move(_pos, para) => {
                    newData = newData.move_to((para.get(0).unwrap() * x, para.get(1).unwrap() * y));
                }

                Command::Line(_pos, para) => {
                    newData = newData.line_to((para.get(0).unwrap() * x, para.get(1).unwrap() * y));
                }

                // HorizontalLine, VerticalLine, QuadraticCurve, SmoothQuadraticCurve
                // SmoothCubicCurve, EllipticalArc, CubicCurve

                Command::Close => {}

                _ => {  unimplemented!() }
            }
        }

        self.path = Some(newData.close());
    }

    fn stretch_to(&mut self, x: f32, y: f32) {
        unimplemented!();
    }
    
    fn reflect(&mut self, p1x: f32, p1y: f32, p2x: f32, p2y: f32) {
        // get line properties
        let slope = (p2y - p1y) / (p2x - p1x);
        let intercept = p1y - slope * p1x;
        // reflect the center
        self.center.0 = self.center.0 - (2.0 * (slope * self.center.1 - self.center.0 + intercept)) / (slope.powi(2) + 1.0);
        self.center.1 = self.center.1 + slope * ((2.0 * (slope * self.center.1 - self.center.0 + intercept)) / (slope.powi(2) + 1.0))- 2.0 * intercept;

    }

    fn warp(&mut self, function: String, freq: f32, ampl: f32) {
        if(function == "ripple") {

        }
    }
        
    fn hue_shift(&mut self, amount: f32) {
        // Convert RGB to HSL
        let r = self.outline_color.0 as f32 / 255.0;
        let g = self.outline_color.1 as f32 / 255.0;
        let b = self.outline_color.2 as f32 / 255.0;
    
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let chroma = max - min;
    
        let hue = if chroma == 0.0 {
            0.0
        } else if max == r {
            60.0 * ((g - b) / chroma % 6.0)
        } else if max == g {
            60.0 * ((b - r) / chroma + 2.0)
        } else {
            60.0 * ((r - g) / chroma + 4.0)
        };
    
        let hue_shifted = (hue + amount) % 360.0;
        let lightness = 0.5 * (max + min);
        let saturation = if chroma == 0.0 {
            0.0
        } else {
            chroma / (1.0 - (2.0 * lightness - 1.0).abs())
        };

        // Convert back to RGB
        let c = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
        let x = c * (1.0 - ((hue_shifted / 60.0) % 2.0 - 1.0).abs());
        let m = lightness - c / 2.0;

        let (r, g, b) = if hue_shifted < 60.0 {
            (c, x, 0.0)
        } else if hue_shifted < 120.0 {
            (x, c, 0.0)
        } else if hue_shifted < 180.0 {
            (0.0, c, x)
        } else if hue_shifted < 240.0 {
            (0.0, x, c)
        } else if hue_shifted < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        // new rgb colors
        let rn = ((r + m) * 255.0) as u8;
        let gn = ((g + m) * 255.0) as u8;
        let bn = ((b + m) * 255.0) as u8;
    
        self.outline_color = (rn, gn, bn);
        // println!("{:#?}", self.outline_color);
    }

    fn update(&mut self) {
        let o_color = format!("#{:02x?}{:02x?}{:02x?}", self.outline_color.0, self.outline_color.1, self.outline_color.2);
        let rotate = format!("rotate({} {} {})", self.rotation, self.point_of_rotation.0, self.point_of_rotation.1);
        self.svg = Some(Path::new()
                    .set("fill", "none")
                    .set("stroke", o_color)
                    .set("stroke-width", 1)
                    .set("transform", rotate)
                    .set("d", self.path.clone().unwrap()));
                    
    }
}