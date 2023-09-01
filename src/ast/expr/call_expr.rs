use std::{any::Any, fmt::Debug};

use crate::{
    ast::{arg::Arg, types::Type},
    utils::join_list,
};

use super::{Expr, ExprKind, ExprTrait};

#[derive(Clone)]
pub struct CallExpr {
    pub postfix_expr: Expr,
    pub args: Vec<Arg>,
    pub r#type: Option<Type>,
}

impl CallExpr {
    pub fn new(postfix_expr: Expr, args: Vec<Arg>) -> Self {
        Self {
            postfix_expr,
            args,
            r#type: None,
        }
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

    fn clone_box(&self) -> Box<dyn ExprTrait> {
        Box::new(self.clone())
    }

    fn r#type(&self) -> &Option<Type> {
        &self.r#type
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
