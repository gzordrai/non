use std::{
    cell::{RefCell},
    collections::HashMap,
    rc::Rc,
};

#[derive(Debug, Default, Clone)]
pub struct Non {
    fields: HashMap<String, FieldValue>,
    pub parent: Option<Rc<RefCell<Non>>>,
}

impl Non {
    pub fn from_id(id: String) -> Self {
        let mut fields = HashMap::new();
        fields.insert("id".to_string(), FieldValue::Litteral(id));
        Non {
            fields,
            ..Default::default()
        }
    }

    pub fn id(&self) -> String {
        match self.fields.get("id").unwrap() {
            FieldValue::Litteral(id) => id.clone(),
            _ => panic!("Id must be a litteral."),
        }
    }

    pub fn add_field(&mut self, name: String, value: FieldValue) {
        self.fields.insert(name, value);
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

    pub fn resolve(&mut self) {
        let mut fields = HashMap::new();
        for (field_name, field_value) in &self.fields {
            let value = self.resolve_field(field_value.clone());
            fields.insert(field_name.clone(), FieldValue::Litteral(value));
        }
        self.fields = fields;
    }

    fn get_field_value(&self, field_name: String) -> String {
        let field = self.fields.get(&field_name).unwrap();
        self.resolve_field(field.clone())
    }

    fn serialize_field_value(&self, field_value: &FieldValue) -> String {
        match field_value {
            FieldValue::Litteral(v) => {
                if v == "@" || v.starts_with('@') {
                    v.clone()
                } else {
                    format!("'{}'", v)
                }
            }
            FieldValue::Vec(field_values) => field_values
                .iter()
                .map(|fv| self.serialize_field_value(fv))
                .collect::<Vec<_>>()
                .join(" "),
            FieldValue::FieldReference(reference) => {
                format!(".{}", reference)
            }
            FieldValue::ObjRef(non, field_name) => {
                format!("{}.{}", non.borrow().id(), field_name)
            }
        }
    }

    pub fn to_custom_format(&self) -> String {
        let mut result = String::new();
        let id = self.id();

        if let Some(parent_ref) = &self.parent {
            let parent = parent_ref.borrow();
            result.push_str(&format!("{}: {}\n", id, parent.id()));

            for (key, value) in &self.fields {
                if key == "id" {
                    continue;
                }

                let should_serialize = if let Some(parent_value) = parent.fields.get(key) {
                    !self.values_equal(value, parent_value)
                } else {
                    true
                };

                if should_serialize {
                    result.push_str(&format!(".{} {}\n", key, self.serialize_field_value(value)));
                }
            }
        } else {
            result.push_str(&format!("{}:\n", id));

            for (key, value) in &self.fields {
                if key == "id" {
                    continue;
                }
                result.push_str(&format!(".{} {}\n", key, self.serialize_field_value(value)));
            }
        }

        result
    }

    fn values_equal(&self, v1: &FieldValue, v2: &FieldValue) -> bool {
        match (v1, v2) {
            (FieldValue::Litteral(a), FieldValue::Litteral(b)) => a == b,
            (FieldValue::FieldReference(a), FieldValue::FieldReference(b)) => a == b,
            (FieldValue::Vec(a), FieldValue::Vec(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                a.iter().zip(b.iter()).all(|(x, y)| self.values_equal(x, y))
            }
            (FieldValue::ObjRef(obj1, field1), FieldValue::ObjRef(obj2, field2)) => {
                obj1.borrow().id() == obj2.borrow().id() && field1 == field2
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FieldValue {
    Litteral(String),
    Vec(Vec<FieldValue>),
    FieldReference(String),
    ObjRef(Rc<RefCell<Non>>, String),
}

pub fn serialize_non_collection(nons: Vec<&Rc<RefCell<Non>>>) -> String {
    nons.iter()
        .map(|n| n.borrow().to_custom_format())
        .collect::<Vec<_>>()
        .join("\n")
}
