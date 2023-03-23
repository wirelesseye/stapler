use std::{env, fs::File};

use lexer::Lexer;
use token::Token;

use crate::token::TokenKind;

mod char_reader;
mod lexer;
mod token;

fn main() {
    let filename = env::args().nth(1).expect("invalid file name");
    let file = File::open(filename).unwrap();

    let mut lexer = Lexer::new(&file);
    let mut token: Token;
    loop {
        token = lexer.next_token();
        println!("{:?}", token);
        
        if matches!(token.kind(), TokenKind::EOF) {
            break;
        }
    }
}
