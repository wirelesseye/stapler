mod ast;
mod compiler;
mod char_reader;
mod lexer;
mod parser;
mod token;
mod utils;

use std::{env, fs::File};
use crate::compiler::Compiler;

fn main() {
    let filename = env::args().nth(1).expect("invalid file name");
    let file = File::open(&filename).unwrap();

    let compiler = Compiler::new();
    compiler.compile(&file, filename);
}
