use std::{any::Any, fmt::Debug};

use crate::{ast::param::Param, utils::join_list};

use super::{Type, TypeKind, TypeTrait};

#[derive(Clone)]
pub struct FuncType {
    pub return_type: Type,
    pub params: Vec<Param>,
    pub is_var_args: bool,
}

impl FuncType {
    pub fn new(return_type: Type, params: Vec<Param>, is_var_args: bool) -> Self {
        Self {
            return_type,
            params,
            is_var_args,
        }
    }
}

impl TypeTrait for FuncType {
    fn kind(&self) -> TypeKind {
        TypeKind::Func
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn TypeTrait> {
        Box::new(self.clone())
    }
}

impl Debug for FuncType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        if self.is_var_args {
            write!(
                f,
                "{}",
                join_list(&self.params[..self.params.len() - 1], ", ")
            )?;
            write!(f, ", ...{:?}", self.params[self.params.len() - 1])?;
        } else {
            write!(f, "{}", join_list(&self.params, ", "))?;
        }
        write!(f, ") -> {:?}", self.return_type)?;

        Ok(())
    }
}
