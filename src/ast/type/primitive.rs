use std::fmt::{Debug, Formatter};
use crate::ast::{Type, TypeKind};

pub enum PrimitiveType {
    I8,
    I32,
    I64,
}

impl Debug for PrimitiveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PrimitiveType::I8 => "i8",
            PrimitiveType::I32 => "i32",
            PrimitiveType::I64 => "i64",
        })
    }
}

impl Type for PrimitiveType {
    fn type_kind(&self) -> TypeKind {
        TypeKind::Primitive
    }
}