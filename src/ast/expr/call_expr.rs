use std::{any::Any, fmt::Debug};

use crate::{ast::arg::Arg, utils::join_list};

use super::{ExprKind, ExprTrait, PostfixExpr};

pub struct CallExpr {
    pub postfix_expr: PostfixExpr,
    pub args: Vec<Arg>,
}

impl CallExpr {
    pub fn new(postfix_expr: PostfixExpr, args: Vec<Arg>) -> Self {
        Self { postfix_expr, args }
    }
}

impl ExprTrait for CallExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Call
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Debug for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}({})",
            self.postfix_expr,
            join_list(&self.args, ", ")
        )
    }
}
