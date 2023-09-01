use std::fmt::Debug;

use super::expr::Expr;

pub struct Arg {
    pub expr: Expr,
}

impl Arg {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }
}

impl Debug for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.expr)
    }
}
