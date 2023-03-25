mod decl;
mod expr;
mod r#extern;

use std::fmt::{self, Debug, Formatter};
use crate::asts::ASTTrait;

pub use decl::*;
pub use expr::*;
pub use r#extern::*;

pub trait StmtTrait: ASTTrait {}

pub struct StmtAST {
    inner: Box<dyn StmtTrait>,
}

impl StmtAST {
    pub fn new(inner: Box<dyn StmtTrait>) -> Self {
        Self {
            inner
        }
    }
}

#[macro_export]
macro_rules! stmt_ast {
    ($inner:expr) => {
        StmtAST::new(Box::new($inner))
    }
}

impl Debug for StmtAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl ASTTrait for StmtAST {}