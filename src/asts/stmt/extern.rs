use std::fmt::{Debug, Formatter};
use crate::asts::{ASTTrait, BlockAST};
use crate::asts::stmt::StmtTrait;

pub struct ExternStmt {
    body: BlockAST
}

impl ExternStmt {
    pub fn new(body: BlockAST) -> Self {
        Self {
            body
        }
    }
}

impl Debug for ExternStmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "extern {:?}", self.body)
    }
}

impl ASTTrait for ExternStmt {}

impl StmtTrait for ExternStmt {}