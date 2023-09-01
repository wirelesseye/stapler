use std::fmt::Debug;

use crate::ast::{ident::Ident, types::Type};

use super::{ExprKind, ExprTrait};

#[derive(Clone)]
pub struct IdentExpr {
    pub ident: Ident,
    pub r#type: Option<Type>,
}

impl IdentExpr {
    pub fn new(ident: Ident) -> Self {
        Self { ident, r#type: None }
    }
}

impl ExprTrait for IdentExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Ident
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn clone_box(&self) -> Box<dyn ExprTrait> {
        Box::new(self.clone())
    }

    fn r#type(&self) -> &Option<Type> {
        &self.r#type
    }
}

impl Debug for IdentExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.ident)
    }
}
