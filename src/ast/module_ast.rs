use std::fmt::Debug;

use crate::utils::join_list;

use super::stmt::Stmt;

pub struct ModuleAST {
    pub name: String,
    pub stmts: Vec<Stmt>,
}

impl ModuleAST {
    pub fn new(name: String, stmts: Vec<Stmt>) -> Self {
        Self { name, stmts }
    }
}

impl Debug for ModuleAST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", join_list(&self.stmts, "\n"))
    }
}
