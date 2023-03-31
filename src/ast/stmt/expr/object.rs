use std::fmt::{Debug, Formatter};
use crate::ast::{Expr, ExprKind, IdentExpr};

pub struct ObjectExpr {
    ident: IdentExpr,
    parent: Option<Box<ObjectExpr>>,
}

impl ObjectExpr {
    pub fn new(ident: IdentExpr, parent: Option<ObjectExpr>) -> Self {
        Self {
            ident,
            parent: if let Some(parent) = parent {
                Some(Box::new(parent))
            } else {
                None
            }
        }
    }
}

impl Debug for ObjectExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(p) = &self.parent {
            write!(f, "{:?}.", p)?;
        }

        write!(f, "{:?}", self.ident)?;
        Ok(())
    }
}

impl Expr for ObjectExpr {
    fn expr_kind(&self) -> ExprKind {
        ExprKind::Object
    }
}