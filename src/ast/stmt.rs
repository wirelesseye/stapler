mod expr;
mod r#extern;
mod decl;

use std::fmt::{Debug, Formatter};

pub use expr::*;
pub use decl::*;
pub use r#extern::*;

pub trait Stmt : Debug {
    fn stmt_kind(&self) -> StmtKind;
}

pub enum StmtKind {
    Expr,
    Extern,
    Decl,
}

pub struct StmtAST {
    inner: Box<dyn Stmt>
}

impl From<ExprStmt> for StmtAST {
    fn from(value: ExprStmt) -> Self {
        Self {
            inner: Box::new(value)
        }
    }
}

impl From<DeclStmt> for StmtAST {
    fn from(value: DeclStmt) -> Self {
        Self {
            inner: Box::new(value)
        }
    }
}

impl From<ExternStmt> for StmtAST {
    fn from(value: ExternStmt) -> Self {
        Self {
            inner: Box::new(value)
        }
    }
}

impl Debug for StmtAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}
