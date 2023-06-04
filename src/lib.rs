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
use crate::art::name;
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
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn debug_log(message: &str) {
    log(message);
}

use std::string::String;

#[wasm_bindgen]
pub fn process_file(content: &str) -> String{
    process(content);
   let svg = name();
    svg
}

// fn open_file(path: &str) -> Result<String, std::io::Error> {
//     let mut file = File::open(path)?;
//     let mut rdin = String::new();
//     file.read_to_string(&mut rdin)?;
//     Ok(rdin)
// }

fn process(content: &str) {
    let mut lex = Lexer::new(String::from(content));
    let tokens = lex.lex();

    for t in &tokens {
        log(format!("{:#?}", t).as_str());
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    log(format!("{:#?}", ast).as_str());
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

  