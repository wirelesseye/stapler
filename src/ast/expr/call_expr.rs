use std::{any::Any, fmt::Debug};

use crate::{ast::{arg::Arg, r#ref::Ref}, utils::join_list};

use super::{ExprKind, ExprTrait};

pub struct CallExpr {
    pub r#ref: Ref,
    pub args: Vec<Arg>,
}

impl CallExpr {
    pub fn new(r#ref: Ref, args: Vec<Arg>) -> Self {
        Self { r#ref, args }
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
        write!(f, "{:?}({})", self.r#ref, join_list(&self.args, ", "))
    }
}
