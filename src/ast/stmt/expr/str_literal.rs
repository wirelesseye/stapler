use std::fmt::{Debug, Formatter};
use crate::ast::{Expr, ExprKind};

pub struct StrLiteralExpr {
    value: String
}

impl StrLiteralExpr {
    pub fn new(value: String) -> Self {
        Self {
            value
        }
    }
}

impl Debug for StrLiteralExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}

impl Expr for StrLiteralExpr {
    fn expr_kind(&self) -> ExprKind {
        ExprKind::StrLiteral
    }
}