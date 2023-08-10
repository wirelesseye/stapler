use std::{fmt::Debug, any::Any};

use crate::{ast::param::Param, utils::join_list};

use super::{TypeTrait, TypeKind, Type};

pub struct FuncType {
    return_type: Type,
    params: Vec<Param>,
    is_var_args: bool,
}

impl FuncType {
    pub fn new(return_type: Type, params: Vec<Param>, is_var_args: bool) -> Self {
        Self {
            return_type,
            params,
            is_var_args
        }
    }

    pub fn return_type(&self) -> &Type {
        &self.return_type
    }

    pub fn params(&self) -> &[Param] {
        &self.params
    }

    pub fn is_var_args(&self) -> bool {
        self.is_var_args
    }
}

impl TypeTrait for FuncType {
    fn kind(&self) -> TypeKind {
        TypeKind::Func
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Debug for FuncType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}", join_list(&self.params, ", "))?;
        if self.is_var_args() {
            write!(f, ", ...")?;
        }
        write!(f, ") -> {:?}", self.return_type)?;
        Ok(())
    }
}