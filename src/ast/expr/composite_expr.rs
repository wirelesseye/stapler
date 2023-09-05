use std::fmt::Debug;

use crate::ast::types::Type;

use super::{Expr, ExprKind, ExprTrait};

#[derive(Clone)]
pub struct CompositeExpr {
    pub fields: Vec<(String, Expr)>,
    pub r#type: Option<Type>,
}

impl CompositeExpr {
    pub fn new(fields: Vec<(String, Expr)>) -> Self {
        Self {
            fields,
            r#type: None,
        }
    }
}

impl ExprTrait for CompositeExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Composite
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

    fn r#type(&self) -> &Option<crate::ast::types::Type> {
        &self.r#type
    }
}

impl Debug for CompositeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;
        for field in &self.fields {
            write!(f, "{}: {:?}\n", field.0, field.1)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
