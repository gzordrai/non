use std::{collections::HashMap, rc::Rc};

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Non {
    pub id: String,
    fields: HashMap<String, FieldValue>,
    pub parent: Option<Rc<Non>>,
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

    pub fn resolve(&mut self) {

    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FieldValue {
    Litteral(String),
    Vec(Vec<FieldValue>),
    FieldReference(String),
    ObjRef(Rc<Non>, String),
}
