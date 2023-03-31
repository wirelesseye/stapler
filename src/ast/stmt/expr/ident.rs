use std::fmt::{Debug, Formatter};
use crate::ast::ExprKind;
use crate::ast::stmt::expr::Expr;

pub struct IdentExpr {
    value: String
}

impl IdentExpr {
    pub fn new(value: String) -> Self {
        Self {
            value
        }
    }
}

impl Debug for IdentExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Expr for IdentExpr {
    fn expr_kind(&self) -> ExprKind {
        ExprKind::Ident
    }
}