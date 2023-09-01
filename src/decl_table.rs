use crate::ast::types::Type;

pub struct DeclEntry {
    pub name: String,
    pub decl_id: u64,
    pub level: u64,
    pub r#type: Option<Type>,
}

pub struct DeclTable {
    level: u64,
    refs: Vec<DeclEntry>,
    next_id: u64,
}

impl DeclTable {
    pub fn new() -> Self {
        Self {
            level: 1,
            refs: Vec::new(),
            next_id: 0,
        }
    }

    pub fn open_scope(&mut self) {
        self.level += 1;
    }

    pub fn close_scope(&mut self) {
        let mut last = self.refs.last();
        while last.is_some() && last.unwrap().level == self.level {
            self.refs.pop();
            last = self.refs.last();
        }
        self.level -= 1;
    }

    pub fn push(&mut self, name: &str, r#type: Option<Type>) -> u64 {
        let decl_id = self.next_id;
        self.refs.push(DeclEntry {
            name: name.to_owned(),
            decl_id,
            level: self.level,
            r#type,
        });
        self.next_id += 1;
        decl_id
    }

    pub fn retrieve(&self, name: &str) -> Option<&DeclEntry> {
        for entry in self.refs.iter().rev() {
            if entry.name == name {
                return Some(entry);
            }
        }
        None
    }

    pub fn retrieve_same_level(&self, name: &str) -> Option<&DeclEntry> {
        for entry in self.refs.iter().rev() {
            if entry.level != self.level {
                return None;
            }
            if entry.name == name {
                return Some(entry);
            }
        }
        None
    }
}
