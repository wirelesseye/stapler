use std::fmt::{self, Debug, Formatter};
use crate::asts::{ASTTrait, StmtAST};

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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let children_output: Vec<String> = self.children.iter()
            .map(|stmt| {format!("{:?}", stmt)})
            .collect();
        let joined = children_output.join("\n");
        write!(f, "{{\n{}\n}}", joined)
    }
}

impl ASTTrait for BlockAST {}