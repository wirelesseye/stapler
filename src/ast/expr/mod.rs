mod call;
mod ident;
mod int_literal;
mod str_literal;

use std::{fmt::Debug, any::Any};

pub use call::*;
pub use ident::*;
pub use int_literal::*;
pub use str_literal::*;

pub enum ExprKind {
    Call,
    IntLiteral,
    StrLiteral,
    Ident,
}

pub trait ExprTrait : Debug {
    fn kind(&self) -> ExprKind;

    fn as_any(&self) -> &dyn Any;
}

pub struct Expr {
    inner: Box<dyn ExprTrait>,
}

impl Expr {
    pub fn kind(&self) -> ExprKind {
        self.inner.kind()
    }

    pub fn cast<T>(&self) -> &T where T: ExprTrait + 'static {
        self.inner.as_any().downcast_ref::<T>().unwrap()
    }
}

impl<T> From<T> for Expr
where
    T: ExprTrait + 'static,
{
    fn from(value: T) -> Self {
        Self {
            inner: Box::new(value),
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}