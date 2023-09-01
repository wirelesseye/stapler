use std::{any::Any, fmt::Debug};

use crate::ast::expr::PostfixExpr;

use super::{TypeKind, TypeTrait};

#[derive(Clone)]
pub struct RefType {
    pub postfix_expr: PostfixExpr,
}

impl RefType {
    pub fn new(postfix_expr: PostfixExpr) -> Self {
        Self { postfix_expr }
    }
}

impl TypeTrait for RefType {
    fn kind(&self) -> TypeKind {
        TypeKind::Ref
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn TypeTrait> {
        Box::new(self.clone())
    }
}

impl Debug for RefType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.postfix_expr)
    }
}
