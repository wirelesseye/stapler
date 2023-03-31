mod call;
mod ident;
mod object;
mod int_literal;
mod str_literal;

use std::fmt::{Debug, Formatter};
use crate::ast::{Stmt, StmtKind};

pub use call::*;
pub use ident::*;
pub use object::*;
pub use int_literal::*;
pub use str_literal::*;

pub trait Expr : Debug {
    fn expr_kind(&self) -> ExprKind;
}

pub enum ExprKind {
    Call,
    Ident,
    Object,
    IntLiteral,
    StrLiteral,
}

pub struct ExprStmt {
    inner: Box<dyn Expr>
}

impl ExprStmt {
    pub fn inner(&self) -> &Box<dyn Expr> {
        &self.inner
    }
}

impl From<IdentExpr> for ExprStmt {
    fn from(value: IdentExpr) -> Self {
        Self {
            inner: Box::new(value)
        }
    }
}

impl From<ObjectExpr> for ExprStmt {
    fn from(value: ObjectExpr) -> Self {
        Self {
            inner: Box::new(value)
        }
    }
}

impl From<CallExpr> for ExprStmt {
    fn from(value: CallExpr) -> Self {
        Self {
            inner: Box::new(value)
        }
    }
}

impl From<IntLiteralExpr> for ExprStmt {
    fn from(value: IntLiteralExpr) -> Self {
        Self {
            inner: Box::new(value)
        }
    }
}

impl From<StrLiteralExpr> for ExprStmt {
    fn from(value: StrLiteralExpr) -> Self {
        Self {
            inner: Box::new(value)
        }
    }
}

impl Debug for ExprStmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner())
    }
}

impl Stmt for ExprStmt {
    fn stmt_kind(&self) -> StmtKind {
        StmtKind::Expr
    }
}