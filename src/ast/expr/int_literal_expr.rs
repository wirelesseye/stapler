use std::{any::Any, fmt::Debug};

use crate::ast::types::{Type, IntType};

use super::{ExprKind, ExprTrait};

#[derive(Clone)]
pub struct IntLiteralExpr {
    pub value: String,
    pub r#type: Option<Type>,
}

impl IntLiteralExpr {
    pub fn new(value: String) -> Self {
        Self {
            value,
            r#type: Some(IntType::I32.into()),
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

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn ExprTrait> {
        Box::new(self.clone())
    }

    fn r#type(&self) -> &Option<Type> {
        &self.r#type
    }
}

impl Debug for IntLiteralExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
