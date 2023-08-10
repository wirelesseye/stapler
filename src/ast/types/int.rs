use std::{fmt::Debug, any::Any};

use super::{TypeTrait, TypeKind};

#[derive(Debug)]
pub enum IntType {
    I8,
    I32,
    I64,
}

impl TypeTrait for IntType {
    fn kind(&self) -> TypeKind {
        TypeKind::Int
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
