use std::fmt::{Debug, Formatter};
use crate::ast::StmtAST;
use crate::utils::format_list;

pub struct BlockAST {
    children: Vec<StmtAST>
}

impl BlockAST {
    pub fn new(children: Vec<StmtAST>) -> Self {
        Self {
            children
        }
    }
}

impl Debug for BlockAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n{}\n}}", format_list(&self.children, "\n"))
    }
}
