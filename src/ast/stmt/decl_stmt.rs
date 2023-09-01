use std::{fmt::Debug, any::Any};

use crate::{ast::decl::Decl, utils::join_list};

use super::{StmtKind, StmtTrait};

pub struct DeclStmt {
    pub decls: Vec<Decl>,
    pub is_export: bool,
}

impl DeclStmt {
    pub fn new(decls: Vec<Decl>, is_export: bool) -> Self {
        Self { decls, is_export }
    }
}

impl StmtTrait for DeclStmt {
    fn kind(&self) -> StmtKind {
        StmtKind::Decl
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Debug for DeclStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_export {
            write!(f, "export ")?;
        }
        write!(f, "{}", join_list(&self.decls, "\n"))?;
        Ok(())
    }
}