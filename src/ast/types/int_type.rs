use std::{fmt::Debug, any::Any};

use super::{TypeTrait, TypeKind};

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

impl Debug for IntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I8 => write!(f, "i8"),
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
        }
    }
}
