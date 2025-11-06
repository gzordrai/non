use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Non {
    id: String,
    fields: HashMap<String, FieldValue>,
    pub parent: Option<String>,
}

#[allow(dead_code)]
impl Non {
    pub fn from_id(id: String) -> Self {
        Non {
            id,
            ..Default::default()
        }
    }

    pub fn add_field(&mut self, name: String, value: FieldValue) {
        self.fields.insert(name, value);
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FieldValue {
    Litteral(String),
    Vec(Vec<FieldValue>),
    FieldReference(String),
    ObjRef(String, String),
}
