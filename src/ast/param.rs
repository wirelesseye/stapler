use std::fmt::Debug;

use super::types::Type;

pub struct Param {
    pub name: String,
    pub r#type: Type,
}

impl Param {
    pub fn new(name: String, r#type: Type) -> Self {
        Self { name, r#type }
    }
}

impl Debug for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.name, self.r#type)
    }
}
