use std::{fmt::Debug, any::Any};

use super::{ExprKind, ExprTrait};

pub struct StrLiteralExpr {
    value: String,
}

impl StrLiteralExpr {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ExprTrait for StrLiteralExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::StrLiteral
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Debug for StrLiteralExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}
