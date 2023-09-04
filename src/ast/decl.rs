use std::fmt::{Debug, Formatter};

use super::{expr::Expr, types::Type};

pub struct Decl {
    pub name: String,
    pub r#type: Option<Type>,
    pub value: Option<Expr>,
    pub value_id: Option<u64>,
}

impl Decl {
    pub fn new(name: String, r#type: Option<Type>, value: Option<Expr>) -> Self {
        Self {
            name,
            r#type,
            value,
            value_id: None,
        }
    }
}

impl Debug for Decl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "let {}", self.name)?;
        if let Some(value_id) = self.value_id {
            write!(f, "({})", value_id)?;
        }
        if let Some(r#type) = &self.r#type {
            write!(f, ": {:?}", r#type)?;
        }
        if let Some(value) = &self.value {
            write!(f, " = {:?}", value)?;
        }
        Ok(())
    }
}
