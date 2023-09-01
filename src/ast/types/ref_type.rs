use std::{any::Any, fmt::Debug};

use crate::ast::ident::Ident;

use super::{TypeTrait, TypeKind};

pub struct RefType {
    pub ident: Ident
}

impl RefType {
    pub fn new(ident: Ident) -> Self {
        Self {
            ident
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
}

impl Debug for RefType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.ident)
    }
}