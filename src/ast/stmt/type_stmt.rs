use std::fmt::Debug;

use crate::ast::{ident::Ident, types::Type};

use super::{StmtKind, StmtTrait};

pub struct TypeStmt {
    pub ident: Ident,
    pub r#type: Type,
}

impl TypeStmt {
    pub fn new(ident: Ident, r#type: Type) -> Self {
        Self { ident, r#type }
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
        write!(f, "type {:?} {:?}", self.ident, self.r#type)
    }
}
