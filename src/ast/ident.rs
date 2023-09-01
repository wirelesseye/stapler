use std::fmt::Debug;

pub struct Ident {
    pub value: String,
}

impl Ident {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
