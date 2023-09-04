use std::{any::Any, fmt::Debug};

use crate::ast::expr::Expr;

use super::{TypeKind, TypeTrait};

#[derive(Clone)]
pub struct RefType {
    pub expr: Expr,
    pub type_id: Option<u64>,
}

impl RefType {
    pub fn new(expr: Expr) -> Self {
        Self {
            expr,
            type_id: None,
        }
    }
}

impl TypeTrait for RefType {
    fn kind(&self) -> TypeKind {
        TypeKind::Ref
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn TypeTrait> {
        Box::new(self.clone())
    }
}

impl Debug for RefType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.expr)?;
        if let Some(type_id) = self.type_id {
            write!(f, "({})", type_id)?;
        }
        Ok(())
    }
}
