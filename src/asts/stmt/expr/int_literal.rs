use std::fmt::{Debug, Formatter};
use crate::asts::stmt::expr::ExprTrait;
use crate::asts::{ASTTrait, StmtTrait};

pub struct IntLiteralExpr {
    value: String
}

impl IntLiteralExpr {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string()
        }
    }
}

impl Debug for IntLiteralExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl StmtTrait for IntLiteralExpr {}

impl ASTTrait for IntLiteralExpr {}

impl ExprTrait for IntLiteralExpr {}