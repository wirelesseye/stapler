use std::fmt::{Debug, Formatter};
use crate::asts::{ASTTrait, ExprStmt, IdentExpr, StmtTrait};
use crate::asts::stmt::expr::ExprTrait;

pub struct CallExpr {
    ident: IdentExpr,
    arg_list: Vec<ExprStmt>,
}

impl CallExpr {
    pub fn new(ident: IdentExpr, arg_list: Vec<ExprStmt>) -> Self {
        Self {
            ident,
            arg_list
        }
    }
}

impl Debug for CallExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({:?})", self.ident, self.arg_list)
    }
}

impl StmtTrait for CallExpr {}

impl ASTTrait for CallExpr {}

impl ExprTrait for CallExpr {}