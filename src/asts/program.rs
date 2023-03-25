use std::fmt;
use std::fmt::Formatter;
use crate::asts::ASTTrait;
use crate::asts::block::BlockAST;

pub struct ProgramAST {
    body: BlockAST
}

impl ProgramAST {
    pub fn new(body: BlockAST) -> Self {
        Self {
            body
        }
    }
}

impl fmt::Debug for ProgramAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.body)
    }
}

impl ASTTrait for ProgramAST {}