use std::fmt::{Debug, Formatter};
use crate::ast::{DeclStmt, Stmt, StmtKind};
use crate::utils::format_list;

pub struct ExternStmt {
    decl_list: Vec<DeclStmt>
}

impl ExternStmt {
    pub fn new(decl_list: Vec<DeclStmt>) -> Self {
        Self {
            decl_list
        }
    }
}

impl Debug for ExternStmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "extern {{\n{}\n}}", format_list(&self.decl_list, "\n"))
    }
}

impl Stmt for ExternStmt {
    fn stmt_kind(&self) -> StmtKind {
        StmtKind::Extern
    }
}