use std::fmt::{Debug, Formatter};
use crate::asts::stmt::expr::ExprTrait;
use crate::asts::{ASTTrait, StmtTrait};

pub struct StrLiteralExpr {
    value: String
}

impl StrLiteralExpr {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string()
        }
    }
}

impl Debug for StrLiteralExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}

impl StmtTrait for StrLiteralExpr {}

impl ASTTrait for StrLiteralExpr {}

impl ExprTrait for StrLiteralExpr {}