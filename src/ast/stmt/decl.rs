use std::fmt::{Debug, Formatter};
use crate::ast::{Stmt, StmtKind, TypeAST};
use crate::ast::stmt::expr::{ExprStmt, IdentExpr};
use crate::utils::format_list;

pub struct Decl {
    ident: IdentExpr,
    r#type: Option<TypeAST>,
    value: Option<ExprStmt>,
}

impl Decl {
    pub fn new(ident: IdentExpr, r#type: Option<TypeAST>, value: Option<ExprStmt>) -> Self {
        Self {
            ident,
            r#type,
            value,
        }
    }
}

impl Debug for Decl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.ident)?;

        if let Some(t) = &self.r#type {
            write!(f, ": {:?}", t)?;
        }

        if let Some(v) = &self.value {
            write!(f, " = {:?}", v)?;
        }

        Ok(())
    }
}

pub struct DeclStmt {
    decls: Vec<Decl>
}

impl DeclStmt {
    pub fn new(decls: Vec<Decl>) -> Self {
        Self {
            decls
        }
    }
}

impl Debug for DeclStmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "let {}", format_list(&self.decls, ", "))
    }
}

impl Stmt for DeclStmt {
    fn stmt_kind(&self) -> StmtKind {
        StmtKind::Decl
    }
}