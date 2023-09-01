use std::{fmt::Debug, any::Any};

use crate::ast::r#ref::Ref;

use super::{ExprTrait, ExprKind};

pub struct RefExpr {
    pub r#ref: Ref
}

impl RefExpr {
    pub fn new(r#ref: Ref) -> Self {
        Self {
            r#ref
        }
    }
}

impl ExprTrait for RefExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Ref
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Debug for RefExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.r#ref)
    }
}
