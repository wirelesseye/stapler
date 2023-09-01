use std::{any::Any, fmt::Debug};

use crate::ast::expr::Expr;

use super::{StmtTrait, StmtKind};

pub struct ReturnStmt {
    expr: Option<Expr>
}

impl ReturnStmt {
    pub fn new(expr: Option<Expr>) -> Self {
        Self {
            expr
        }
    }

    pub fn expr(&self) -> &Option<Expr> {
        &self.expr
    }
}

impl StmtTrait for ReturnStmt {
    fn kind(&self) -> StmtKind {
        StmtKind::Return
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Debug for ReturnStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "return {:?}", self.expr)
    }
}