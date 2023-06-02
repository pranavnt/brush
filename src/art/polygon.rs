use std::ops::DerefMut;
use std::ptr::addr_of;

use crate::error::Error;
use svg::node::element::path::{Command, Data, Parameters};
use svg::node::element::tag::Path;
use svg::node::element::{Circle, Line, Path};
use svg::parser::Event;
use svg::Document;

use crate::art::{BPolygon, Drawable, Shape};

impl BPolygon {
    pub fn new(
        x_vals: Vec::<f32>,
        y_vals: Vec::<f32>,
        outline_color: Option<(u8, u8, u8)>,
        outline_width: f32,
        fill_color: (u8, u8, u8, u8)
    ) -> BPolygon {
        let mut xs = 0.0;
        let mut ys = 0.0;

        for x in &x_vals {
            xs += x;
        }

        for y in&y_vals {
            ys += y;
        }

        xs /= x_vals.len() as f32;
        ys /= y_vals.len() as f32;

        let mut newData = Data::new();

        newData = newData.move_to((x_vals[0], y_vals[0]));

        for i in 1..(x_vals.len()) {
            newData = newData.line_to((x_vals[i], y_vals[i]));
        }

        newData = newData.line_to((x_vals[0], y_vals[0]));
        newData = newData.close();

        BPolygon {
            shape: Shape {
                svg: None,
                path: Some(newData),

                circ: None,
                rect: None,

                center: (xs, ys),
                dimensions: (0.0, 0.0),
                fill: fill_color,
                outline_color: outline_color.unwrap_or((0, 0, 0)),
                outline_width: outline_width,
                rotation: 0.0,
                transformation_stack: "".to_string(),
                warp_vals: (0.0, 0.0),
                stretch: (1.0, 1.0),
            },

            points: Vec::<(f32, f32)>::new()
        }
    }
}

impl Drawable for BPolygon {
    fn rotate(&mut self, angle: f32) {
        self.shape.rotate(angle);
    }

    fn rotate_to(&mut self, angle: f32) {
        self.shape.rotate_to(angle);
    }

    fn rotate_about(&mut self, angle: f32, x: f32, y: f32) {
        self.shape.rotate_about(angle, x, y);
    }

    fn shift(&mut self, x: f32, y: f32) {
        self.shape.shift(x, y);
    }

    fn shift_to(&mut self, x: f32, y: f32) {
        self.shape.shift_to(x, y);
    }

    fn stretch(&mut self, x: f32, y: f32) {
        self.shape.stretch(x, y);
    }

    fn stretch_to(&mut self, x: f32, y: f32) {
        self.shape.stretch_to(x, y);
    }

    fn reflect(&mut self, p1x: f32, p1y: f32, p2x: f32, p2y: f32) {
        self.shape.reflect(p1x, p1y, p2x, p2y);
    }

    fn warp(&mut self, freq: f32, ampl: f32) {
        self.shape.warp(freq, ampl);
    }
    fn hue_shift(&mut self, amount: f32) {
        self.shape.hue_shift(amount);
    }

    fn update(&mut self) {
        self.shape.update();
    }
}
