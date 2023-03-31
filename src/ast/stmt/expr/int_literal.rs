use std::fmt::{Debug, Formatter};
use crate::ast::{Expr, ExprKind};

pub struct IntLiteralExpr {
    value: String
}

impl IntLiteralExpr {
    pub fn new(value: String) -> Self {
        Self {
            value
        }
    }
}

impl Debug for IntLiteralExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Expr for IntLiteralExpr {
    fn expr_kind(&self) -> ExprKind {
        ExprKind::IntLiteral
    }
}