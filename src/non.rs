use std::{cell::RefCell, collections::HashMap, rc::Rc};

use serde::{Serialize};

#[derive(Debug, Default, Clone, Serialize)]
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

    pub fn get(&self, field_name: &str) -> Option<String> {
        self.fields().get(field_name).map(|field| self.resolve_field(field.clone()))
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
                str.push_str(self.get(&reference).unwrap().as_str())
            }
            FieldValue::ObjRef(non, field_name) => {
                str.push_str(non.borrow().get(&field_name).unwrap().as_str())
            }
        }
        str
    }

    pub fn _resolve(&mut self) {
        let mut fields = HashMap::new();

        for (field_name, field_value) in &self.fields {
            let value = self.resolve_field(field_value.clone());
            fields.insert(field_name.clone(), FieldValue::Litteral(value));
        }

        self.fields = fields;
    }

    pub fn serialize_non(&self, flat: bool) -> String {
        let mut str = String::new();

        str.push_str(&self.id().to_string());
        str.push(':');

        if let Some(parent_ref) = &self.parent && !flat {
            let parent = parent_ref.borrow();
            str.push(' ');
            str.push_str(&parent.id());
        }
        
        str.push('\n');


        let fields = if flat {
            &self.fields()
        } else {
            &self.fields
        };
        
        for (key, value) in fields {
            if key == "id" {
                continue;
            }

            let serialized_field_value = if flat {
                self.resolve_field(value.clone())
            } else {
                value.to_string()
            };
            str.push_str(&format!(".{} {}\n", key, serialized_field_value));
        }

        str
    }

    pub fn serialize_json(&self, flat: bool) -> String {
        let mut str = String::new();

        str.push_str("{\n");
        str.push_str("\t\"id\": \"");
        str.push_str(&self.id());
        str.push_str("\"");

        if let Some(parent) = &self.parent && !flat {
            str.push_str(",\n\t");
            str.push_str("\"parent\": \"");
            str.push_str(&parent.borrow().id());
            str.push('\"');
        }

        let fields = if flat {
            &self.fields()
        } else {
            &self.fields
        };


        if fields.len() > 1 { // id always here
            str.push_str(",\n\t\"fields\":\n\t{\n");

            let fields_str = fields.iter()
                .filter(|(name, _)| "id" != *name)
                .map(|(field_name, value)| {
                    if flat {
                        format!("\t\t\"{}\": \"{}\"", field_name, self.resolve_field(value.clone()))
                    } else {
                        format!("\t\t\"{}\": \"{}\"", field_name, value.to_string())
                    }
                })
                .collect::<Vec<_>>()
                .join(",\n");
            str.push_str(&fields_str);
            str.push_str("\n\t}");
        }

        str.push_str("\n}");
        str
    }

    pub fn serialize_yaml(&self, flat: bool) -> String {
        let mut str = String::new();

        str.push_str(&self.id());
        str.push_str(":");

        if let Some(parent) = &self.parent {
            str.push_str("\n\t");
            str.push_str("parent: ");
            str.push_str(&parent.borrow().id());
        }

        if !self.fields.is_empty() {
            str.push('\n');
        }

        let fields = if flat {
            &self.fields()
        } else {
            &self.fields
        };

        let fields_str = fields.iter()
            .filter(|(name, _)| "id" != *name)
            .map(|(field_name, value)| {
                if flat {
                    format!("\t{}: {}", field_name, self.resolve_field(value.clone()))
                } else {
                    format!("\t{}: {}", field_name, value.to_string())
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        str.push_str(&fields_str);

        str.push_str("\n");
        str
    }

    fn fields(&self) -> HashMap<String, FieldValue> {
        let mut map = HashMap::new();
        if let Some(parent) = &self.parent {
            map.extend(parent.borrow().fields().into_iter());
        }
        map.extend(self.fields.clone().into_iter());
        map
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum FieldValue {
    Litteral(String),
    Vec(Vec<FieldValue>),
    FieldReference(String),
    ObjRef(Rc<RefCell<Non>>, String),
}

impl FieldValue {
    pub fn to_string(&self) -> String {
        match self {
            FieldValue::Litteral(str) => str.clone(),
            FieldValue::Vec(field_values) => field_values.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(" "),
            FieldValue::FieldReference(reference) => {
                if reference == "id" {
                    "@"
                } else {
                    reference
                }.to_string()
            },
            FieldValue::ObjRef(reference, field) => format!("{}.{}", reference.borrow().id(), field.clone()),
        }
    }
}