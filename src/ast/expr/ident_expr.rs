use std::{fmt::Debug, any::Any};

use crate::ast::ident::Ident;

use super::{ExprTrait, ExprKind};

pub struct IdentExpr {
    ident: Ident
}

impl IdentExpr {
    pub fn new(ident: Ident) -> Self {
        Self {
            ident
        }
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }
}

impl ExprTrait for IdentExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Ident
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Debug for IdentExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.ident)
    }
}
