use std::fmt::Debug;

use crate::ast::types::Type;

use super::{StmtKind, StmtTrait};

pub struct TypeStmt {
    pub lhs: Type,
    pub rhs: Type,
}

impl TypeStmt {
    pub fn new(lhs: Type, rhs: Type) -> Self {
        Self { lhs, rhs }
    }
}

impl StmtTrait for TypeStmt {
    fn kind(&self) -> StmtKind {
        StmtKind::Type
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Debug for TypeStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "typedef {:?} {:?}", self.lhs, self.rhs)
    }
}
