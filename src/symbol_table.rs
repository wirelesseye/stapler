use crate::ast::types::Type;

pub struct TypeEntry {
    pub type_id: u64,
    pub name: String,
    pub level: u64,
}

pub struct ValueEntry {
    pub value_id: u64,
    pub name: String,
    pub r#type: Option<Type>,
    pub level: u64,
}

pub struct SymbolTable {
    level: u64,
    values: Vec<ValueEntry>,
    types: Vec<TypeEntry>,
    next_id: u64,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            level: 1,
            values: Vec::new(),
            types: Vec::new(),
            next_id: 0,
        }
    }

    pub fn open_scope(&mut self) {
        self.level += 1;
    }

    pub fn close_scope(&mut self) {
        let mut last = self.values.last();
        while last.is_some() && last.unwrap().level == self.level {
            self.values.pop();
            last = self.values.last();
        }
        self.level -= 1;
    }

    pub fn push_value(&mut self, name: &str, r#type: Option<Type>) -> u64 {
        let id = self.next_id;
        self.values.push(ValueEntry {
            value_id: id,
            name: name.to_owned(),
            r#type,
            level: self.level,
        });
        self.next_id += 1;
        id
    }

    pub fn push_type(&mut self, name: &str) -> u64 {
        let id = self.next_id;
        self.types.push(TypeEntry {
            type_id: id,
            name: name.to_owned(),
            level: self.level,
        });
        self.next_id += 1;
        id
    }

    pub fn retrieve_value(&self, name: &str) -> Option<&ValueEntry> {
        for entry in self.values.iter().rev() {
            if entry.name == name {
                return Some(entry);
            }
        }
        None
    }

    pub fn retrieve_type(&self, name: &str) -> Option<&TypeEntry> {
        for entry in self.types.iter().rev() {
            if entry.name == name {
                return Some(entry);
            }
        }
        None
    }

    pub fn retrieve_value_same_level(&self, name: &str) -> Option<&ValueEntry> {
        for entry in self.values.iter().rev() {
            if entry.level != self.level {
                return None;
            }
            if entry.name == name {
                return Some(entry);
            }
        }
        None
    }

    pub fn retrieve_type_same_level(&self, name: &str) -> Option<&TypeEntry> {
        for entry in self.types.iter().rev() {
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
