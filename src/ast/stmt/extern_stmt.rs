use std::{fmt::Debug, any::Any};

use crate::utils::join_list;

use super::{decl_stmt::DeclStmt, StmtKind, StmtTrait};

pub struct ExternStmt {
    pub decl_stmts: Vec<DeclStmt>,
}

impl ExternStmt {
    pub fn new(decl_stmts: Vec<DeclStmt>) -> Self {
        Self {
            decl_stmts
        }
    }
}

impl StmtTrait for ExternStmt {
    fn kind(&self) -> StmtKind {
        StmtKind::Extern
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Debug for ExternStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "extern {{\n{}\n}}", join_list(&self.decl_stmts, "\n"))
    }
}