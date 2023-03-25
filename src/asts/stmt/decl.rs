use std::fmt::{Debug, Formatter};
use crate::asts::{ASTTrait, IdentExpr, TypeAST};
use crate::asts::stmt::StmtTrait;

pub struct DeclStmt {
    ident: IdentExpr,
    r#type: Option<TypeAST>
}

impl DeclStmt {
    pub fn new(ident: IdentExpr, r#type: Option<TypeAST>) -> Self {
        Self {
            ident,
            r#type,
        }
    }
}

impl Debug for DeclStmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(r#type) = &self.r#type {
            write!(f, "let {:?}: {:?}", self.ident, r#type)
        } else {
            write!(f, "let {:?}", self.ident)
        }
    }
}

impl ASTTrait for DeclStmt {}

impl StmtTrait for DeclStmt {}