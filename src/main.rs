use std::fs::File;
use std::io::Seek;

use checker::Checker;
use clap::Parser as ClapParser;
use codegen::Codegen;
use parser::Parser;

use crate::lexer::Lexer;
use crate::token::TokenKind;

mod ast;
mod checker;
mod char_reader;
mod codegen;
mod lexer;
mod parser;
mod decl_table;
mod token;
mod utils;

#[derive(ClapParser, Debug)]
struct Args {
    source: String,

    /// Print tokens
    #[arg(short = 't', long)]
    tokens: bool,

    /// Print AST
    #[arg(short = 'a', long)]
    ast: bool,

    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut file = File::open(&args.source).unwrap();

    if args.tokens {
        let mut lexer = Lexer::new(&file);
        loop {
            let token = lexer.next_token();
            if token.is_kind(TokenKind::EOF) {
                break;
            }
            println!("{:?}", token);
        }
        println!();
        file.seek(std::io::SeekFrom::Start(0)).unwrap();
        return;
    }

    let mut parser = Parser::new(&args.source, &file);
    let mut module_ast = parser.parse();

    let mut checker = Checker::new();
    checker.check(&mut module_ast);

    if args.ast {
        println!("{:?}\n", module_ast);
        return;
    }

    let codegen = Codegen::new();
    codegen.build_module(&module_ast, args.output.as_deref());
}
