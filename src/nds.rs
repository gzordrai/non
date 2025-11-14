use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use crate::{args::OutputFormat, non::Non};

pub struct NonDefs {
    nons: HashMap<String, Rc<RefCell<Non>>>,
    format: OutputFormat,
    flat: bool,
}

impl NonDefs {
    pub fn new(nons: HashMap<String, Rc<RefCell<Non>>>, format: OutputFormat, flat: bool) -> Self {
        NonDefs { nons, format, flat }
    }

    pub fn builder() -> NonDefsBuilder {
        NonDefsBuilder::default()
    }

    pub fn at(&self, id: &str) -> Option<Ref<'_, Non>> {
        self.nons.get(id).map(|n| n.borrow())
    }

    pub fn serialize(&self) -> String {
        let mut str = String::new();
        match self.format {
            OutputFormat::Json => {
                str.push_str("[\n");
                let nons = self
                    .nons
                    .values()
                    .map(|n| n.borrow().serialize_json(self.flat))
                    .collect::<Vec<_>>()
                    .join(",\n");
                str.push_str(&nons);
                str.push_str("\n]");
            }
            OutputFormat::Yaml => {
                let yaml = self
                    .nons
                    .values()
                    .map(|n| n.borrow().serialize_yaml(self.flat))
                    .collect::<Vec<_>>()
                    .join("\n");
                str.push_str(&yaml);
            }
            OutputFormat::Non => {
                let nons = self
                    .nons
                    .values()
                    .map(|n| n.borrow().serialize_non(self.flat))
                    .collect::<Vec<_>>()
                    .join("\n");
                str.push_str(&nons);
            }
        }
        str
    }
}

#[derive(Default)]
pub struct NonDefsBuilder {
    nons: Option<HashMap<String, Rc<RefCell<Non>>>>,
    format: Option<OutputFormat>,
    flat: Option<bool>,
}

impl NonDefsBuilder {
    pub fn build(self) -> NonDefs {
        NonDefs::new(self.nons.unwrap(), self.format.unwrap(), self.flat.unwrap())
    }

    pub fn nons(mut self, nons: HashMap<String, Rc<RefCell<Non>>>) -> Self {
        self.nons = Some(nons);
        self
    }

    pub fn format(mut self, format: OutputFormat) -> Self {
        self.format = Some(format);
        self
    }

    pub fn flat(mut self, flat: bool) -> Self {
        self.flat = Some(flat);
        self
    }
}
