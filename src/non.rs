use crate::{
    error::Result,
    parser::{Field, FieldValue},
};

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

    pub fn get(&self, k: &str) -> Option<String> {
        let field = self.fields.iter().find(|n| n.name == k)?;

        self.resolve_field_value(&field.value)
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

    fn resolve_field_value(&self, value: &FieldValue) -> Option<String> {
        println!("{:?}", &self.fields);
        println!("{:?}", value);

        match value {
            FieldValue::String(s) => Some(s.clone()),
            FieldValue::Reference => Some(self.name.clone()),
            FieldValue::SelfFieldRef(field_name) => self.get(field_name),
            FieldValue::FieldRef(non_name, field_name) => self.get(field_name),
            FieldValue::Concat(values) => {
                let mut result = String::new();

                for v in values {
                    result.push_str(&self.resolve_field_value(v)?);
                }

                Some(result)
            }
        }
    }

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
