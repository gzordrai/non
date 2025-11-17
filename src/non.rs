use crate::{error::Result, parser::Field};

#[derive(Debug, Default, Clone)]
pub struct Non {
    pub name: String,
    pub parents: Vec<String>,
    pub fields: Vec<Field>,
}

impl Non {
    pub fn new(name: String, parents: Vec<String>, fields: Vec<Field>) -> Self {
        Self {
            name,
            parents,
            fields,
        }
    }

    pub fn id(&self) -> &String {
        &self.name
    }

    pub fn get(&self, k: &str) -> Option<&Field> {
        self.fields.iter().find(|n| n.name == k)
    }

    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }

    pub fn merge_fields(&mut self, other: &Non) -> Result<()> {
        for other_field in &other.fields {
            if let Some(existing) = self.fields.iter_mut().find(|f| f.name == other_field.name) {
                existing.value = other_field.value.clone();
            } else {
                self.fields.push(other_field.clone());
            }
        }

        Ok(())
    }

    // pub fn get(&self, field_name: &str) -> Option<String> {
    //     self.fields()
    //         .get(field_name)
    //         .map(|field| self.resolve_field(field.clone()))
    // }

    // pub fn add_field(&mut self, name: String, value: FieldValue) {
    //     self.fields.insert(name, value);
    // }

    // pub fn resolve_field(&self, field_value: FieldValue) -> String {
    //     let mut str = String::new();
    //     match field_value {
    //         FieldValue::Litteral(v) => str.push_str(&v),
    //         FieldValue::Vec(field_values) => {
    //             for field_value in field_values {
    //                 str.push_str(self.resolve_field(field_value).as_str());
    //             }
    //         }
    //         FieldValue::FieldReference(reference) => {
    //             if reference == "id" {
    //                 str.push_str(&self.id());
    //             } else {
    //                 str.push_str(self.get(&reference).unwrap().as_str())
    //             }
    //         }
    //         FieldValue::ObjRef(non, field_name) => {
    //             str.push_str(non.borrow().get(&field_name).unwrap().as_str())
    //         }
    //     }
    //     str
    // }

    // pub fn union(&self, other: Ref<'_, Non>) -> Result<Non, String> {
    //     let fields = self.fields();
    //     let other_fields = other.fields();

    //     for (name, value) in &fields {
    //         if let Some(other_value) = other_fields.get(name) {
    //             if other_value != value {
    //                 return Err(format!("Duplicated field '{}' without same value.", name));
    //             }
    //         }
    //     }

    //     for (name, value) in &other_fields {
    //         if let Some(other_value) = fields.get(name) {
    //             if *other_value != *value {
    //                 return Err(format!("Duplicated field '{}' without same value.", name));
    //             }
    //         }
    //     }

    //     let mut union_fields = self.fields();
    //     union_fields.extend(other_fields);

    //     let mut parents = self.parents.iter().cloned().collect::<Vec<_>>();
    //     parents.extend(other.parents.iter().cloned());

    //     // filter parents to avoid duplications
    //     let mut seen = HashSet::new();
    //     parents.retain(|p| {
    //         let ptr = Rc::as_ptr(p);
    //         seen.insert(ptr)
    //     });

    //     Ok(Non::new(self.id(), union_fields, parents))
    // }

    // pub fn serialize_non(&self, flat: bool) -> String {
    //     let mut str = String::new();

    //     str.push_str(&self.id().to_string());
    //     str.push(':');

    //     if !flat {
    //         for parent_ref in &self.parents {
    //             let parent = parent_ref.borrow();
    //             str.push(' ');
    //             str.push_str(&parent.id());
    //         }
    //     }

    //     str.push('\n');

    //     let fields = if flat { &self.fields() } else { &self.fields };

    //     for (key, value) in fields {
    //         let serialized_field_value = if flat {
    //             self.resolve_field(value.clone())
    //         } else {
    //             value.to_string()
    //         };
    //         str.push_str(&format!(".{} {}\n", key, serialized_field_value));
    //     }

    //     str
    // }

    // pub fn serialize_json(&self, flat: bool) -> String {
    //     let mut str = String::new();

    //     str.push_str("{\n");
    //     str.push_str("\t\"id\": \"");
    //     str.push_str(&self.id());
    //     str.push_str("\"");

    //     let fields = if flat { &self.fields() } else { &self.fields };

    //     if !(fields.is_empty() && self.parents.is_empty()) {
    //         str.push(',');
    //     }

    //     if !flat && !self.parents.is_empty() {
    //         str.push_str("\n\t\"parents\": [\n");
    //         let parent_str = self
    //             .parents
    //             .iter()
    //             .map(|parent| format!("\t\t\"{}\"", parent.borrow().id()))
    //             .collect::<Vec<String>>()
    //             .join(",\n");
    //         str.push_str(&parent_str);
    //         str.push_str("\n\t]");
    //         if !fields.is_empty() {
    //             str.push(',');
    //         }
    //     }

    //     if !fields.is_empty() {
    //         str.push_str("\n\t\"fields\":\n\t{\n");

    //         let fields_str = fields
    //             .iter()
    //             .map(|(field_name, value)| {
    //                 if flat {
    //                     format!(
    //                         "\t\t\"{}\": \"{}\"",
    //                         field_name,
    //                         self.resolve_field(value.clone())
    //                     )
    //                 } else {
    //                     format!("\t\t\"{}\": \"{}\"", field_name, value.to_string())
    //                 }
    //             })
    //             .collect::<Vec<_>>()
    //             .join(",\n");
    //         str.push_str(&fields_str);
    //         str.push_str("\n\t}");
    //     }

    //     str.push_str("\n}");
    //     str
    // }

    // pub fn serialize_yaml(&self, flat: bool) -> String {
    //     let mut str = String::new();

    //     str.push_str(&self.id());
    //     str.push_str(":");

    //     if !self.parents.is_empty() {
    //         str.push_str("\n\tparents:\n\t");
    //         let parent_str = self
    //             .parents
    //             .iter()
    //             .map(|parent| format!("  - {}", parent.borrow().id()))
    //             .collect::<Vec<String>>()
    //             .join("\n\t");
    //         str.push_str(&parent_str);
    //     }

    //     if !self.fields.is_empty() {
    //         str.push('\n');
    //     }

    //     let fields = if flat { &self.fields() } else { &self.fields };

    //     let fields_str = fields
    //         .iter()
    //         .map(|(field_name, value)| {
    //             if flat {
    //                 format!("\t{}: {}", field_name, self.resolve_field(value.clone()))
    //             } else {
    //                 format!("\t{}: {}", field_name, value.to_string())
    //             }
    //         })
    //         .collect::<Vec<_>>()
    //         .join("\n");
    //     str.push_str(&fields_str);

    //     str.push_str("\n");
    //     str
    // }
}
