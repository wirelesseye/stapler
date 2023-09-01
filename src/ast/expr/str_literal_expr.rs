use std::{any::Any, fmt::Debug};

use crate::ast::{
    ident::Ident,
    types::{RefType, Type},
};

use super::{ExprKind, ExprTrait, IdentExpr};

#[derive(Clone)]
pub struct StrLiteralExpr {
    pub value: String,
    pub r#type: Option<Type>,
}

impl StrLiteralExpr {
    pub fn new(value: String) -> Self {
        Self {
            value,
            r#type: Some(
                RefType::new(IdentExpr::new(Ident::new("String".to_string())).into()).into(),
            ),
        }
    }
}

impl ExprTrait for StrLiteralExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::StrLiteral
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

impl Debug for StrLiteralExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}
