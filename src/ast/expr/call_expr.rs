use std::{fmt::Debug, any::Any};

use crate::{
    ast::{arg::Arg, ident::Ident},
    utils::join_list,
};

use super::{ExprKind, ExprTrait};

pub struct CallExpr {
    ident: Ident,
    args: Vec<Arg>,
}

impl CallExpr {
    pub fn new(ident: Ident, args: Vec<Arg>) -> Self {
        Self { ident, args }
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn args(&self) -> &[Arg] {
        &self.args
    }
}

impl ExprTrait for CallExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Call
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Debug for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self.ident, join_list(&self.args, ", "))
    }
}
