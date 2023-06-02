use std::ops::DerefMut;
use std::ptr::addr_of;

use crate::error::Error;
use svg::node::element::path::{Command, Data, Parameters};
use svg::node::element::tag::Path;
use svg::node::element::{Circle, Line, Path};
use svg::parser::Event;
use svg::Document;

use crate::art::{Drawable, Shape};

fn rgb_shift(color: (u8, u8, u8), amount: f32) -> (u8, u8, u8) {
    // Convert RGB to HSL
    let r = color.0 as f32 / 255.0;
    let g = color.1 as f32 / 255.0;
    let b = color.2 as f32 / 255.0;

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

    (rn, gn, bn)
}

impl Drawable for Shape {
    fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
        self.transformation_stack +=
            &format!(" rotate({} {} {})", angle, self.center.0, self.center.1);
    }

    fn rotate_to(&mut self, angle: f32) {
        self.transformation_stack += &format!(
            " rotate({} {} {})",
            (angle - self.rotation),
            self.center.0,
            self.center.1
        );
        self.rotation = angle;
    }

    fn rotate_about(&mut self, angle: f32, x: f32, y: f32) {
        self.transformation_stack += &format!(" rotate({} {} {})", angle, x, y);
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

                _ => {
                    unimplemented!()
                }
            }
        }

        self.path = Some(newData.close());
    }

    fn shift_to(&mut self, x: f32, y: f32) {
        self.center.0 = x;
        self.center.1 = y;

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

                _ => {
                    unimplemented!()
                }
            }
        }

        self.path = Some(newData.close());
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

                _ => {
                    unimplemented!()
                }
            }
        }

        self.path = Some(newData.close());
    }

    fn stretch_to(&mut self, x: f32, y: f32) {
        unimplemented!();
    }

    fn reflect(&mut self, p1x: f32, p1y: f32, p2x: f32, p2y: f32) {
        if p1x == p2x {
            let distance = (self.center.0 - p1x);
            self.center.0 = p1x - distance;
        } else if p1y == p2y {
            self.center.1 = 2.0 * p1y - self.center.1;
        } else {
            let slope = (p2y - p1y) / (p2x - p1x);

            let y_intercept = p1y - slope * p1x;

            let perp_slope = -1.0 / slope;

            let perp_y_intercept = self.center.1 - perp_slope * self.center.0;

            let x_intersect = (perp_y_intercept - y_intercept) / (slope - perp_slope);
            let y_intersect = slope * x_intersect + y_intercept;

            let reflected_x = 2.0 * x_intersect - self.center.0;
            let reflected_y = 2.0 * y_intersect - self.center.1;

            self.center.0 = reflected_x;
            self.center.1 = reflected_y;
        }
    }

    fn warp(&mut self, freq: f32, ampl: f32) {
        self.warp_vals = (freq, ampl);
    }

    fn hue_shift(&mut self, amount: f32) {
        let new_fill = rgb_shift((self.fill.0, self.fill.1, self.fill.2), amount);
        let new_out = rgb_shift(self.outline_color, amount);

        self.fill = (new_fill.0, new_fill.1, new_fill.2, self.fill.3);
        self.outline_color = new_out;
        // println!("{:#?}", self.outline_color);
    }

    fn update(&mut self) {
        let o_color = format!(
            "#{:02x?}{:02x?}{:02x?}",
            self.outline_color.0, self.outline_color.1, self.outline_color.2
        );

        self.svg = Some(
            Path::new()
                .set("fill", "none")
                .set("stroke", o_color)
                .set("stroke-width", self.outline_width)
                .set("transform", self.transformation_stack.clone())
                .set("d", self.path.clone().unwrap()),
        );
    }
}
