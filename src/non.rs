use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Non {
    pub id: String,
    fields: HashMap<String, FieldValue>,
    pub parent: Option<Rc<RefCell<Non>>>,
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
        let mut fields = HashMap::new();
        for (field_name, field_value) in &self.fields {
            let value = self.resolve_field(field_value.clone());
            fields.insert(field_name.clone(), FieldValue::Litteral(value));
        }
        self.fields = fields;
    }

    fn get_field_value(&self, field_name: String) -> String {
        if field_name.eq("id") {
            self.id.clone()
        } else {
            let field = self.fields.get(&field_name).unwrap();
            self.resolve_field(field.clone())
        }
    }

    pub fn resolve_field(&self, field_value: FieldValue) -> String {
        let mut str = String::new();
        match field_value {
            FieldValue::Litteral(v) => str.push_str(&v),
            FieldValue::Vec(field_values) => {
                for field_value in field_values {
                    str.push_str(self.resolve_field(field_value).as_str());
                }
            }
            FieldValue::FieldReference(reference) => {
                str.push_str(self.get_field_value(reference).as_str())
            }
            FieldValue::ObjRef(non, field_name) => {
                str.push_str(non.borrow().get_field_value(field_name).as_str())
            }
        }
        str
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FieldValue {
    Litteral(String),
    Vec(Vec<FieldValue>),
    FieldReference(String),
    ObjRef(Rc<RefCell<Non>>, String),
}