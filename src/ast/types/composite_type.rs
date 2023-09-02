use std::fmt::Debug;

use crate::{ast::param::Param, utils::join_list};

use super::{TypeTrait, TypeKind};

#[derive(Clone)]
pub struct CompositeType {
    pub fields: Vec<Param>,
    pub is_restrict: bool,
}

impl CompositeType {
    pub fn new(fields: Vec<Param>, is_restrict: bool) -> Self {
        Self {
            fields,
            is_restrict
        }
    }
}

impl TypeTrait for CompositeType {
    fn kind(&self) -> TypeKind {
        TypeKind::Composite
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

impl Debug for CompositeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_restrict {
            write!(f, "restrict ")?;
        }
        write!(
            f,
            "{{\n{}\n}}",
            join_list(&self.fields, "\n")
        )?;
        Ok(())
    }
}