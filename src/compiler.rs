use std::fs::File;
use inkwell::context::Context;
use crate::parser::Parser;

pub struct Compiler {
    context: Context
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            context: Context::create()
        }
    }

    pub fn compile(&self, input: &File, module_name: String) {
        let mut parser = Parser::new(input);
        let module = parser.parse(module_name);
        println!("{:?}", module);
    }
}