use std::{fmt::Debug, any::Any};

use super::{Type, TypeKind, TypeTrait};

#[derive(Clone)]
pub struct PtrType {
    pub pointee: Type,
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

    fn clone_box(&self) -> Box<dyn TypeTrait> {
        Box::new(self.clone())
    }
}

impl Debug for PtrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "*{:?}", self.pointee)
    }
}
