use std::fmt::Debug;

use crate::utils::join_list;

use super::stmt::Stmt;

pub struct ModuleAST {
    name: String,
    stmts: Vec<Stmt>,
}

impl ModuleAST {
    pub fn new(name: String, stmts: Vec<Stmt>) -> Self {
        Self { name, stmts }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn stmts(&self) -> &[Stmt] {
        &self.stmts
    }
}

impl Debug for ModuleAST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", join_list(&self.stmts, "\n"))
    }
}
