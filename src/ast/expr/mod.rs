mod call_expr;
mod int_literal_expr;
mod postfix_expr;
mod str_literal_expr;

use std::{any::Any, fmt::Debug};

pub use call_expr::*;
pub use int_literal_expr::*;
pub use postfix_expr::*;
pub use str_literal_expr::*;

pub enum ExprKind {
    Call,
    IntLiteral,
    Postfix,
    StrLiteral,
}

pub trait ExprTrait: Debug {
    fn kind(&self) -> ExprKind;

    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;
}

pub struct Expr {
    inner: Box<dyn ExprTrait>,
}

impl Expr {
    pub fn kind(&self) -> ExprKind {
        self.inner.kind()
    }

    pub fn cast<T>(&self) -> &T
    where
        T: ExprTrait + 'static,
    {
        self.inner.as_any().downcast_ref::<T>().unwrap()
    }

    pub fn cast_mut<T>(&mut self) -> &mut T
    where
        T: ExprTrait + 'static,
    {
        self.inner.as_mut_any().downcast_mut::<T>().unwrap()
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
