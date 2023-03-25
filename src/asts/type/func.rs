use std::fmt::{Debug, Formatter};
use crate::asts::{ASTTrait, ParamAST, TypeAST, TypeTrait};

pub struct FuncType {
    param_list: Vec<ParamAST>,
    return_type: Box<TypeAST>,
}

impl FuncType {
    pub fn new(param_list: Vec<ParamAST>, return_type: TypeAST) -> Self {
        Self {
            param_list,
            return_type: Box::new(return_type),
        }
    }
}

impl Debug for FuncType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}) -> {:?}", self.param_list, self.return_type)
    }
}

impl ASTTrait for FuncType {}

impl TypeTrait for FuncType {}