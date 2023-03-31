use std::fmt::{Debug, Formatter};
use crate::ast::BlockAST;

pub struct ModuleAST {
    name: String,
    block: BlockAST,
}

impl ModuleAST {
    pub fn new(name: String, block: BlockAST) -> Self {
        Self {
            name,
            block,
        }
    }
}

impl Debug for ModuleAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.name, self.block)
    }
}
