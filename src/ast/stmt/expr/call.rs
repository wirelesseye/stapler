use std::fmt::{Debug, Formatter};
use crate::ast::{Expr, ExprKind, ExprStmt, ObjectExpr};
use crate::utils::format_list;

pub struct CallExpr {
    object: ObjectExpr,
    arg_list: Vec<ExprStmt>,
}

impl CallExpr {
    pub fn new(object: ObjectExpr, arg_list: Vec<ExprStmt>) -> Self {
        Self {
            object,
            arg_list,
        }
    }
}

impl Debug for CallExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self.object, format_list(&self.arg_list, ", "))
    }
}

impl Expr for CallExpr {
    fn expr_kind(&self) -> ExprKind {
        ExprKind::Call
    }
}