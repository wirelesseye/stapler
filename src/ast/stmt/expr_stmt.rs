use std::{fmt::Debug, any::Any};

use crate::ast::expr::Expr;

use super::{StmtTrait, StmtKind};

pub struct ExprStmt {
    pub expr: Expr
}

impl ExprStmt {
    pub fn new(expr: Expr) -> Self {
        Self {
            expr
        }
    }
}

impl StmtTrait for ExprStmt {
    fn kind(&self) -> StmtKind {
        StmtKind::Expr
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Debug for ExprStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.expr)
    }
}