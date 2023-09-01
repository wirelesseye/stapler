use std::fmt::Debug;

use crate::ast::ident::Ident;

use super::{ExprTrait, ExprKind};

#[derive(Clone)]
pub struct PostfixExpr {
    pub ident: Ident,
    pub child: Option<Box<PostfixExpr>>,
}

impl PostfixExpr {
    pub fn new(ident: Ident, child: Option<PostfixExpr>) -> Self {
        Self {
            ident,
            child: if let Some(child) = child {
                Some(Box::new(child))
            } else {
                None
            },
        }
    }
}

impl ExprTrait for PostfixExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Postfix
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Debug for PostfixExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.ident)?;
        if let Some(child) = &self.child {
            write!(f, "{:?}", child)?;
        }
        Ok(())
    }
}