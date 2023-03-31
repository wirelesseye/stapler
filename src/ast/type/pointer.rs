use std::fmt::{Debug, Formatter};
use crate::ast::{Type, TypeAST, TypeKind};

pub struct PointerType {
    pointee_type: TypeAST
}

impl PointerType {
    pub fn new(pointee_type: TypeAST) -> Self {
        Self {
            pointee_type
        }
    }
}

impl Debug for PointerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "*{:?}", self.pointee_type)
    }
}

impl Type for PointerType {
    fn type_kind(&self) -> TypeKind {
        TypeKind::Pointer
    }
}