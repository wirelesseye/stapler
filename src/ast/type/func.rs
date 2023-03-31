use std::fmt::{Debug, Formatter};
use crate::ast::{ParamAST, Type, TypeAST, TypeKind};
use crate::utils::format_list;

pub struct FuncType {
    return_type: TypeAST,
    param_list: Vec<ParamAST>
}

impl FuncType {
    pub fn new(return_type: TypeAST, param_list: Vec<ParamAST>) -> Self {
        Self {
            return_type,
            param_list,
        }
    }
}

impl Debug for FuncType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) -> {:?}", format_list(&self.param_list, ", "), self.return_type)
    }
}

impl Type for FuncType {
    fn type_kind(&self) -> TypeKind {
        TypeKind::Func
    }
}