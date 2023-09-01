use std::fmt::Debug;

pub struct Ref {
    pub name: String,
    pub child: Option<Box<Ref>>,
}

impl Ref {
    pub fn new(name: String, child: Option<Ref>) -> Self {
        Self {
            name,
            child: if let Some(child) = child {
                Some(Box::new(child))
            } else {
                None
            },
        }
    }
}

impl Debug for Ref {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(child) = &self.child {
            write!(f, ".{:?}", child)?;
        }
        Ok(())
    }
}