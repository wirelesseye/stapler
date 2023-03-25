mod asts;
mod char_reader;
mod lexer;
mod parser;
mod token;

use std::{env, fs::File};

use lexer::Lexer;
use token::{Token, TokenKind};
use crate::parser::Parser;

fn main() {
    let filename = env::args().nth(1).expect("invalid file name");
    let file = File::open(filename).unwrap();

    let mut parser = Parser::new(&file);
    let program = parser.parse();

    println!("{:?}", program);
}
