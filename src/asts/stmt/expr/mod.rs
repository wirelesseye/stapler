mod assign;
mod ident;
mod call;
mod int_literal;
mod str_literal;

use std::fmt;
use std::fmt::{Debug, Formatter};
use crate::asts::stmt::StmtTrait;

pub use assign::AssignExpr;
pub use ident::IdentExpr;
pub use call::CallExpr;
pub use int_literal::IntLiteralExpr;
pub use str_literal::StrLiteralExpr;
use crate::asts::ASTTrait;

pub trait ExprTrait: StmtTrait {}

pub struct ExprStmt {
    inner: Box<dyn ExprTrait>,
}

impl ExprStmt {
    pub fn new(inner: Box<dyn ExprTrait>) -> Self {
        Self {
            inner
        }
    }
}

#[macro_export]
macro_rules! expr_stmt {
    ($inner:expr) => {
        ExprStmt::new(Box::new($inner))
    }
}

impl Debug for ExprStmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl ASTTrait for ExprStmt {}

impl StmtTrait for ExprStmt {}