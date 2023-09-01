mod call_expr;
mod ident_expr;
mod int_literal_expr;
mod member_expr;
mod str_literal_expr;

use std::{any::Any, fmt::Debug};

pub use call_expr::*;
pub use ident_expr::*;
pub use int_literal_expr::*;
pub use member_expr::*;
pub use str_literal_expr::*;

use super::types::Type;

pub enum ExprKind {
    Call,
    Ident,
    IntLiteral,
    StrLiteral,
    Member,
}

pub trait ExprTrait: Debug {
    fn kind(&self) -> ExprKind;

    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn clone_box(&self) -> Box<dyn ExprTrait>;

    fn r#type(&self) -> &Option<Type>;
}

impl Clone for Box<dyn ExprTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Expr {
    inner: Box<dyn ExprTrait>,
}

impl Expr {
    pub fn kind(&self) -> ExprKind {
        self.inner.kind()
    }

    pub fn r#type(&self) -> &Option<Type> {
        self.inner.r#type()
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
