use std::fmt::{Debug, Formatter};
use crate::ast::{IdentExpr, TypeAST};

pub struct ParamAST {
    ident: IdentExpr,
    r#type: TypeAST,
}

impl ParamAST {
    pub fn new(ident: IdentExpr, r#type: TypeAST) -> Self {
        Self {
            ident,
            r#type,
        }
    }

    pub fn r#type(&self) -> &TypeAST {
        &self.r#type
    }
}

impl Debug for ParamAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.ident, self.r#type)
    }
}