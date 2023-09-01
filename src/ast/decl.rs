use std::fmt::{Debug, Formatter};

use super::{expr::Expr, types::Type};

pub struct Decl {
    pub name: String,
    pub r#type: Option<Type>,
    pub value: Option<Expr>,
}

impl Decl {
    pub fn new(name: String, r#type: Option<Type>, value: Option<Expr>) -> Self {
        Self {
            name,
            r#type,
            value,
        }
    }
}

impl Debug for Decl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "let {}", self.name)?;
        if let Some(r#type) = &self.r#type {
            write!(f, ": {:?}", r#type)?;
        }
        if let Some(value) = &self.value {
            write!(f, " = {:?}", value)?;
        }
        Ok(())
    }
}
