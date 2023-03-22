mod art;
mod ast;
mod error;
mod eval;
mod lexer;
mod parser;
mod tokens;

use crate::eval::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;

use crate::art::draw;
use crate::art::Circle;
use crate::art::Drawable;

use std::env;
use std::fs::File;
use std::io::Read;

use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

fn open_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut rdin = String::new();
    file.read_to_string(&mut rdin)?;
    Ok(rdin)
}

fn main() {
    let mut args = env::args();

    if args.len() > 1 {
        let filename: String = args.nth(1).unwrap();

        match open_file(filename.as_str()) {
            Ok(raw) => {
                let mut lex = Lexer::new(raw);
                let tokens = lex.lex();

                // uncomment to see tokens
                // for t in &tokens {
                //     println!("{:#?}", t);
                // }

                let mut parser = Parser::new(tokens);

                let ast = parser.parse_program();

                println!("{:#?}", ast);

                // let interpreter = Interpreter::new(ast);

                // interpreter.run();

                testing_transform();
            }

            Err(e) => {
                panic!("failed to run {}: {}", filename, e);
            }
        }
    }
}

fn testing_transform() {
    let mut testcircle = Circle::new(500.0, 500.0, 100.0);
    testcircle.shift(100.0, 0.0);
    // 600, 600
    testcircle.stretch(2.0, 2.0);

    let mut shapes = Vec::new();
    shapes.push(testcircle.shape);
    draw(shapes);
}
