use std::fmt::Debug;

pub struct Ident {
    pub name: String,
    pub decl_id: Option<u64>,
}

impl Ident {
    pub fn new(name: String) -> Self {
        Self {
            name,
            decl_id: None,
        }
    }
}

impl Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(decl_id) = &self.decl_id {
            write!(f, "({})", decl_id)?;
        }
        Ok(())
    }
}
