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

#[no_mangle]
#[wasm_bindgen]
pub fn process_file(content: &str) -> String {
    process(content);
}


// fn open_file(path: &str) -> Result<String, std::io::Error> {
//     let mut file = File::open(path)?;
//     let mut rdin = String::new();
//     file.read_to_string(&mut rdin)?;
//     Ok(rdin)
// }

fn process(content: &string) {

    let mut lex = Lexer::new(content);
    let tokens = lex.lex();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();
    let mut interpreter = Interpreter::new(ast);
    interpreter.run();
}

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