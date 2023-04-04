use std::ops::DerefMut;
use std::ptr::addr_of;

use crate::error::Error;
use svg::node::element::path::{Command, Data, Parameters};
use svg::node::element::tag::Path;
use svg::node::element::{Line, Path};
use svg::parser::Event;
use svg::Document;

#[derive(Debug, Clone)]
pub struct Shape {
    pub svg: Path,
    path: Data,
    pub center: (f32, f32),
    dimensions: (f32, f32),
    fill: (u8, u8, u8),
    outline_color: (u8, u8, u8),
    outline_width: f32,
    rotation: f32,
    stretch: (f32, f32),
}

pub trait Drawable {
    fn rotate(&mut self, angle: f32);
    fn rotate_to(&mut self, angle: f32);
    fn shift(&mut self, x: f32, y: f32);
    fn shift_to(&mut self, x: f32, y: f32);
    fn stretch(&mut self, x: f32, y: f32);
    fn stretch_to(&mut self, x: f32, y: f32);
    fn hue_shift(&mut self, amount: f32);
    fn update(&mut self);
}

#[derive(Debug, Clone)]
pub struct Circle {
    pub shape: Shape,
    pub radius: f32,
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    shape: Shape,
    width: f32,
    height: f32,
}

#[derive(Debug, Clone)]
pub struct Polygon {
    shape: Shape,
    points: Vec<(f32, f32)>,
}

#[derive(Debug, Clone)]
pub struct SVG {
    shape: Shape,
    dimensions: (f32, f32),
}

impl Circle {
    pub fn new(x: f32, y: f32, radius: f32, outline_color: Option<(u8, u8, u8)>) -> Circle {
        let mut cdata = Data::new();

        for i in 0..=360 {
            let theta = i as f32;
            let tx = theta.to_radians().cos() * radius + x;
            let ty = theta.to_radians().sin() * radius + y;

            if i == 0 {
                cdata = cdata.move_to((tx, ty));
            } else {
                cdata = cdata.line_to((tx, ty));
            }
        }

        cdata = cdata.close();

        Circle {
            shape: Shape {
                svg: Path::new()
                    .set("fill", "none")
                    .set("stroke", "#000000")
                    .set("stroke-width", 1)
                    .set("d", cdata.clone()),
                path: cdata,
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

    pub fn new_default() -> Circle {
        Circle {
            shape: Shape {
                svg: Path::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 1),
                path: Data::new(),
                center: (0.0, 0.0),
                dimensions: (0.0, 0.0),
                fill: (0, 0, 0),
                outline_color: (0, 0, 0),
                outline_width: 1.0,
                rotation: 0.0,
                stretch: (1.0, 1.0),
            },
            radius: 0.0,
        }
    }
}

impl Drawable for Shape {
    fn rotate(&mut self, angle: f32) {
        unimplemented!();
    }

    fn rotate_to(&mut self, angle: f32) {
        unimplemented!();
    }

    fn shift(&mut self, x: f32, y: f32) {
        self.center.0 += x;
        self.center.1 += y;

        // iterate through the path and shift each point
        let mut cdata = self.path.clone();
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

        self.path = newData.close();

        self.update();
    }

    fn shift_to(&mut self, x: f32, y: f32) {
        unimplemented!();
    }

    fn stretch(&mut self, x: f32, y: f32) {
        self.center.0 *= x;
        self.center.1 *= y;

        let mut cdata = self.path.clone();
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

        self.path = newData.close();

        self.update();
    }

    fn stretch_to(&mut self, x: f32, y: f32) {
        unimplemented!();
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
        self.update();
    }

    fn update(&mut self) {
        let o_color = format!("#{:02x?}{:02x?}{:02x?}", self.outline_color.0, self.outline_color.1, self.outline_color.2);

        self.svg = Path::new()
                    .set("fill", "none")
                    .set("stroke", o_color)
                    .set("stroke-width", 1)
                    .set("d", self.path.clone());
    }
}

impl Drawable for Circle {
    fn rotate(&mut self, angle: f32) {
        unimplemented!();
    }

    fn rotate_to(&mut self, angle: f32) {
        unimplemented!();
    }

    fn shift(&mut self, x: f32, y: f32) {
        // self.shape.center = (self.shape.center.0 + x, self.shape.center.1 + y);
        self.shape.shift(x, y);
    }

    fn shift_to(&mut self, x: f32, y: f32) {
        self.shape.center = (x, y);
    }

    fn stretch(&mut self, x: f32, y: f32) {
        if x == y {
            self.radius *= x;
            self.shape.stretch(x, y);
            //shift to scale about center
            self.shape.shift(self.shape.center.0 / x - self.shape.center.0, self.shape.center.1 / y - self.shape.center.1);
        }
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

pub fn draw(shapes: Vec<Shape>) -> Result<(), Error> {
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
