use std::fmt::{Debug, Formatter};
use crate::asts::stmt::expr::ExprTrait;
use crate::asts::{ASTTrait, StmtTrait};

#[derive(Clone)]
pub struct IdentExpr {
    name: String
}

impl IdentExpr {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
    }
}

impl Debug for IdentExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl StmtTrait for IdentExpr {}

impl ASTTrait for IdentExpr {}

impl ExprTrait for IdentExpr {}