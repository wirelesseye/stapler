use std::fmt::Debug;

use crate::ast::{ident::Ident, types::Type};

use super::{Expr, ExprKind, ExprTrait};

#[derive(Clone)]
pub struct MemberExpr {
    pub postfix_expr: Expr,
    pub member: Ident,
    pub r#type: Option<Type>,
}

impl MemberExpr {
    pub fn new(postfix_expr: Expr, member: Ident) -> Self {
        Self {
            postfix_expr,
            member,
            r#type: None
        }
    }
}

impl ExprTrait for MemberExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Member
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

impl Debug for MemberExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}.{:?}", self.postfix_expr, self.member)
    }
}
