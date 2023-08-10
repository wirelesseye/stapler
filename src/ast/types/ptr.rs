use std::{fmt::Debug, any::Any};

use super::{Type, TypeKind, TypeTrait};

pub struct PtrType {
    pointee: Type,
}

impl PtrType {
    pub fn new(pointee: Type) -> Self {
        Self { pointee }
    }
}

impl TypeTrait for PtrType {
    fn kind(&self) -> TypeKind {
        TypeKind::Ptr
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Debug for PtrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "*{:?}", self.pointee)
    }
}
