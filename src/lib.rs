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
//use crate::art::Circle;
use crate::art::Drawable;

use std::env;
use std::fs::File;
use std::io::Read;
use  std::fmt::Display;

use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;


use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
use std::string::String;

#[wasm_bindgen]
pub extern "C" fn process_file(content: &str) -> String{
    process(content);
    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><text x="10" y="50">{}</text></svg>"#,
        read_art_file()
    );

    // Return the SVG string
    svg
}


// fn open_file(path: &str) -> Result<String, std::io::Error> {
//     let mut file = File::open(path)?;
//     let mut rdin = String::new();
//     file.read_to_string(&mut rdin)?;
//     Ok(rdin)
// }
use std::fs;

pub fn read_art_file() -> String {
    let svg_contents = fs::read_to_string("art.svg");
    svg_contents.unwrap_or_else(|error| error.to_string())
}
fn process(content: &str) {

    let mut lex = Lexer::new(content.to_string());
    let tokens = lex.lex();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();
    let mut interpreter = Interpreter::new(ast);
    interpreter.run();

      // transform_test();
//    let mut args = env::args();

    /*if args.len() > 1 {
        let filename: String = args.nth(1).unwrap();
*/
         /*match open_file(filename.as_str()) {
             Ok(raw) => {
                 let mut lex = Lexer::new(raw);
                 let tokens = lex.lex();

                 // uncomment to see tokens
                 // for t in &tokens {
                 //     println!("{:#?}", t);
                 // }

                 let mut parser = Parser::new(tokens);

                 let ast = parser.parse_program();

                 // println!("{:#?}", ast);

                 let mut interpreter = Interpreter::new(ast);

                 interpreter.run();

                 // transform_test();
             }

             Err(e) => {
                 panic!("failed to run {}: {}", filename, e);
             }
         }
     }
    }
    */
}

  