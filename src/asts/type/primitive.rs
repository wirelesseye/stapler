use std::fmt::{Debug, Formatter};
use crate::asts::{ASTTrait, TypeTrait};

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

impl ASTTrait for PrimitiveType {}

impl TypeTrait for PrimitiveType {}