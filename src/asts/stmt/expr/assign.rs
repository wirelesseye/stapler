use std::fmt::{Debug, Formatter};
use crate::asts::{ASTTrait, ExprStmt, IdentExpr, StmtTrait};
use crate::asts::stmt::expr::ExprTrait;

pub struct AssignExpr {
    lhs: IdentExpr,
    rhs: Box<ExprStmt>,
}

impl AssignExpr {
    pub fn new(lhs: IdentExpr, rhs: ExprStmt) -> Self {
        Self {
            lhs,
            rhs: Box::new(rhs)
        }
    }
}

impl Debug for AssignExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} = {:?}", self.lhs, self.rhs)
    }
}

impl StmtTrait for AssignExpr {}

impl ASTTrait for AssignExpr {}

impl ExprTrait for AssignExpr {}