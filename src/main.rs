use std::{env, fs::File, io::Read};

use crate::lexer::Lexer;

mod lexer;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: {} <file>", args[0]);
    }

    let mut file = match File::open(args[1].to_owned()) {
        Ok(file) => file,
        Err(e) => panic!("Error: couldn't open {} because {}", args[1], e),
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => {}
        Err(e) => panic!("Error: couldn't read {} because {}", args[1], e),
    }

    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize();
    for token in tokens {
        println!("{:?}", token);
    }
}
