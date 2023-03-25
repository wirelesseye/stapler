use std::fmt::{Debug, Formatter};
use crate::asts::{ASTTrait, TypeAST, TypeTrait};

pub struct PointerType {
    pointee: Box<TypeAST>
}

impl PointerType {
    pub fn new(pointee: TypeAST) -> Self {
        Self {
            pointee: Box::new(pointee)
        }
    }
}

impl Debug for PointerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "*{:?}", self.pointee)
    }
}

impl ASTTrait for PointerType {}

impl TypeTrait for PointerType {}