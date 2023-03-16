mod tokens; mod lexer; mod ast; mod parser; mod eval; mod error; mod art;

use std::fs::File;
use std::io::Read;
use std::env;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::eval::Interpreter;

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

                let mut parser = Parser::new(tokens);

                let ast = parser.parse_program();

                println!("{:#?}", ast);

                // let interpreter = Interpreter::new(ast);

                // interpreter.run();
            }

            Err(e) => {
                panic!("failed to run {}: {}", filename, e);
            }
        }
    }
}