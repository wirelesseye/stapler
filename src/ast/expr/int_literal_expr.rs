use std::{fmt::Debug, any::Any};

use super::{ExprTrait, ExprKind};

pub struct IntLiteralExpr {
    value: String
}

impl IntLiteralExpr {
    pub fn new(value: String) -> Self {
        Self {
            value
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ExprTrait for IntLiteralExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::IntLiteral
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Debug for IntLiteralExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}