mod primitive;
mod pointer;
mod func;

use std::fmt::{Debug, Formatter};
use crate::asts::ASTTrait;

pub use primitive::PrimitiveType;
pub use pointer::PointerType;
pub use func::FuncType;

pub trait TypeTrait: ASTTrait {}

pub struct TypeAST {
    inner: Box<dyn TypeTrait>
}

impl TypeAST {
    pub fn new(inner: Box<dyn TypeTrait>) -> Self {
        Self {
            inner
        }
    }
}

#[macro_export]
macro_rules! type_ast {
    ($inner:expr) => {
        TypeAST::new(Box::new($inner))
    }
}

impl Debug for TypeAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl ASTTrait for TypeAST {}