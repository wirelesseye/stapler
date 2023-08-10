use std::fmt::Debug;

use super::{ident::Ident, types::Type};

pub struct Param {
    ident: Ident,
    r#type: Type,
}

impl Param {
    pub fn new(ident: Ident, r#type: Type) -> Self {
        Self { ident, r#type }
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn r#type(&self) -> &Type {
        &self.r#type
    }
}

impl Debug for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.ident, self.r#type)
    }
}