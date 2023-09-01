use std::{fmt::Debug, any::Any};

use crate::{ast::decl::Decl, utils::join_list};

use super::{StmtKind, StmtTrait};

pub struct DeclStmt {
    pub decls: Vec<Decl>,
}

impl DeclStmt {
    pub fn new(decls: Vec<Decl>) -> Self {
        Self { decls }
    }
}

impl StmtTrait for DeclStmt {
    fn kind(&self) -> StmtKind {
        StmtKind::Decl
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Debug for DeclStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", join_list(&self.decls, "\n"))
    }
}