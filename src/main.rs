mod char_reader;
mod lexer;
mod token;

use std::{env, fs::File};

use crate::{lexer::Lexer, token::TokenKind};

fn main() {
    let filename = env::args().nth(1).unwrap();
    let file = File::open(filename).unwrap();

    let mut lexer = Lexer::new(&file);
    loop {
        let token = lexer.next_token();
        if token.is_kind(TokenKind::EOF) {
            break;
        }
        println!("{:?}", token);
    }
}
