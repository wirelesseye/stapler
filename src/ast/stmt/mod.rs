mod decl;
mod expr;
mod r#extern;
mod r#return;

use std::{fmt::Debug, any::Any};

pub use decl::*;
pub use expr::*;
pub use r#extern::*;
pub use r#return::*;

pub enum StmtKind {
    Decl,
    Extern,
    Expr,
    Return,
}

pub trait StmtTrait : Debug {
    fn kind(&self) -> StmtKind;

    fn as_any(&self) -> &dyn Any;
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