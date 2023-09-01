use std::{any::Any, fmt::Debug};

use crate::ast::r#ref::Ref;

use super::{TypeKind, TypeTrait};

pub struct RefType {
    pub r#ref: Ref,
}

impl RefType {
    pub fn new(r#ref: Ref) -> Self {
        Self { r#ref }
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
        write!(f, "{:?}", self.r#ref)
    }
}
