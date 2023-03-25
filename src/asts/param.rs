use std::fmt::{Debug, Formatter};
use crate::asts::{ASTTrait, IdentExpr, TypeAST};

pub struct ParamAST {
    ident: IdentExpr,
    r#type: TypeAST,
}

impl ParamAST {
    pub fn new(ident: IdentExpr, r#type: TypeAST) -> Self {
        Self {
            ident,
            r#type
        }
    }
}

impl Debug for ParamAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.ident, self.r#type)
    }
}

impl ASTTrait for ParamAST {}