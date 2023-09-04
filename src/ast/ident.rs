use std::fmt::Debug;

#[derive(Clone)]
pub struct Ident {
    pub name: String,
    pub symbol_id: Option<u64>,
}

impl Ident {
    pub fn new(name: String) -> Self {
        Self {
            name,
            symbol_id: None,
        }
    }
}

impl Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(value_id) = &self.symbol_id {
            write!(f, "({})", value_id)?;
        }
        Ok(())
    }
}
