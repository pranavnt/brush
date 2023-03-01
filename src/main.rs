use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("listening for file at: {}", std::env::current_dir().unwrap().display());
    let mut filename = String::new();

    io::stdin()
        .read_line(&mut filename)
        .expect("failed to read file name");
    let filename = filename.trim();

    let path = Path::new(&filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(f) => panic!("failed to open {}: {}", display, f),
        Ok(file) => file,
    };

    let mut raw = String::new();
    match file.read_to_string(&mut raw) {
        Err(f) => panic!("failed to read {}: {}", display, f),
        Ok(_) => println!("read raw string data from {}:", display),
    }

    println!("{}", raw);
}

