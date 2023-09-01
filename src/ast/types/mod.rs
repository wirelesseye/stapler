mod array_type;
mod func_type;
mod int_type;
mod ptr_type;
mod ref_type;

use std::{fmt::Debug, any::Any};

pub use array_type::*;
pub use func_type::*;
pub use int_type::*;
pub use ptr_type::*;
pub use ref_type::*;

#[derive(Debug, PartialEq, Eq)]
pub enum TypeKind {
    Array,
    Int,
    Func,
    Ptr,
    Ref,
}

pub trait TypeTrait: Debug {
    fn kind(&self) -> TypeKind;

    fn as_any(&self) -> &dyn Any;
}

pub struct Type {
    inner: Box<dyn TypeTrait>,
}

impl Type {
    pub fn kind(&self) -> TypeKind {
        self.inner.kind()
    }

    pub fn cast<T>(&self) -> &T where T: TypeTrait + 'static {
        self.inner.as_any().downcast_ref::<T>().unwrap()
    }
}

impl<T> From<T> for Type
where
    T: TypeTrait + 'static,
{
    fn from(value: T) -> Self {
        Self {
            inner: Box::new(value),
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}
