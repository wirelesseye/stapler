mod ast;
mod builder;
mod char_reader;
mod lexer;
mod parser;
mod token;
mod utils;

use std::{env, fs::File};
use crate::builder::Builder;

fn main() {
    let filename = env::args().nth(1).expect("invalid file name");
    let file = File::open(&filename).unwrap();

    let builder = Builder::new();
    builder.build(&file, filename);
}
