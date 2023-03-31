mod primitive;
mod pointer;
mod func;

use std::fmt::{Debug, Formatter};

pub use primitive::*;
pub use pointer::*;
pub use func::*;

pub trait Type : Debug {
    fn type_kind(&self) -> TypeKind;
}

pub enum TypeKind {
    Primitive,
    Pointer,
    Func,
}

pub struct TypeAST {
    inner: Box<dyn Type>
}

impl From<PrimitiveType> for TypeAST {
    fn from(value: PrimitiveType) -> Self {
        Self {
            inner: Box::new(value)
        }
    }
}

impl From<PointerType> for TypeAST {
    fn from(value: PointerType) -> Self {
        Self {
            inner: Box::new(value)
        }
    }
}

impl From<FuncType> for TypeAST {
    fn from(value: FuncType) -> Self {
        Self {
            inner: Box::new(value)
        }
    }
}

impl Debug for TypeAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}
