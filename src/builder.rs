use std::fs::File;
use inkwell::context::Context;
use crate::parser::Parser;

pub struct Builder {
    context: Context
}

impl Builder {
    pub fn new() -> Self {
        Self {
            context: Context::create()
        }
    }

    pub fn build(&self, input: &File, name: String) {
        let mut parser = Parser::new(input);
        let module = parser.parse(name);
        println!("{:?}", module);


    }
}