use std::{any::Any, fmt::Debug};

use super::{TypeTrait, TypeKind, Type};

#[derive(Clone)]
pub struct ArrayType {
    pub elem_type: Type,
}

impl ArrayType {
    pub fn new(elem_type: Type) -> Self {
        Self {
            elem_type
        }
    }
}

impl TypeTrait for ArrayType {
    fn kind(&self) -> TypeKind {
        TypeKind::Array
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn TypeTrait> {
        Box::new(self.clone())
    }
}

impl Debug for ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}[]", self.elem_type)
    }
}