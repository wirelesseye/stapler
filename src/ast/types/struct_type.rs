use std::fmt::Debug;

use crate::{ast::param::Param, utils::join_list};

use super::{TypeTrait, TypeKind};

#[derive(Clone)]
pub struct StructType {
    pub fields: Vec<Param>,
    pub is_restrict: bool,
}

impl StructType {
    pub fn new(fields: Vec<Param>, is_restrict: bool) -> Self {
        Self {
            fields,
            is_restrict
        }
    }
}

impl TypeTrait for StructType {
    fn kind(&self) -> TypeKind {
        TypeKind::Struct
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn clone_box(&self) -> Box<dyn TypeTrait> {
        Box::new(self.clone())
    }
}

impl Debug for StructType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_restrict {
            write!(f, "restrict ")?;
        }
        write!(
            f,
            "{{\n{}\n}}",
            join_list(&self.fields, ",\n")
        )?;
        Ok(())
    }
}