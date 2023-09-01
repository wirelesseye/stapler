use std::fmt::{Debug, Formatter};

use super::{ident::Ident, types::Type, expr::Expr};

pub struct Decl {
    pub ident: Ident,
    pub r#type: Option<Type>,
    pub value: Option<Expr>,
}

impl Decl {
    pub fn new(ident: Ident, r#type: Option<Type>, value: Option<Expr>) -> Self {
        Self {
            ident,
            r#type,
            value,
        }
    }
}

impl Debug for Decl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "let {:?}", self.ident)?;
        if let Some(r#type) = &self.r#type {
            write!(f, ": {:?}", r#type)?;
        }
        if let Some(value) = &self.value {
            write!(f, " = {:?}", value)?;
        }
        Ok(())
    }
}