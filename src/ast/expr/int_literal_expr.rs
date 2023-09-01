use std::{fmt::Debug, any::Any};

use super::{ExprTrait, ExprKind};

pub struct IntLiteralExpr {
    pub value: String
}

impl IntLiteralExpr {
    pub fn new(value: String) -> Self {
        Self {
            value
        }
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