mod decl_stmt;
mod expr_stmt;
mod extern_stmt;
mod return_stmt;
mod typedef_stmt;

use std::{fmt::Debug, any::Any};

pub use decl_stmt::*;
pub use expr_stmt::*;
pub use extern_stmt::*;
pub use return_stmt::*;
pub use typedef_stmt::*;

pub enum StmtKind {
    Decl,
    Extern,
    Expr,
    Return,
    Typedef,
}

pub trait StmtTrait : Debug {
    fn kind(&self) -> StmtKind;

    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;
}

pub struct Stmt {
    inner: Box<dyn StmtTrait>,
}

impl Stmt {
    pub fn kind(&self) -> StmtKind {
        self.inner.kind()
    }

    pub fn cast<T>(&self) -> &T where T: StmtTrait + 'static {
        self.inner.as_any().downcast_ref::<T>().unwrap()
    }

    pub fn cast_mut<T>(&mut self) -> &mut T where T: StmtTrait + 'static {
        self.inner.as_mut_any().downcast_mut::<T>().unwrap()
    }
}

impl<T> From<T> for Stmt
where
    T: StmtTrait + 'static,
{
    fn from(value: T) -> Self {
        Self {
            inner: Box::new(value),
        }
    }
}

impl Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}