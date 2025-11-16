use crate::{args::OutputFormat, non::Non};
use std::collections::HashMap;

pub struct NonDefs<'a> {
    nons: HashMap<String, &'a Non<'a>>,
    format: OutputFormat,
    flat: bool,
}

impl<'a> NonDefs<'a> {
    pub fn new(nons: HashMap<String, &'a Non<'a>>, format: OutputFormat, flat: bool) -> Self {
        NonDefs { nons, format, flat }
    }

    pub fn builder() -> NonDefsBuilder<'a> {
        NonDefsBuilder::default()
    }

    pub fn at(&self, id: &str) -> Option<&'a Non<'a>> {
        self.nons.get(id).copied()
    }

    pub fn serialize(&self) -> String {
        let mut result = String::new();

        match self.format {
            OutputFormat::Json => {
                result.push_str("[\n");
                let nons = self
                    .nons
                    .values()
                    .map(|n| n.serialize_json(self.flat))
                    .collect::<Vec<_>>()
                    .join(",\n");
                result.push_str(&nons);
                result.push_str("\n]");
            }
            OutputFormat::Yaml => {
                let yaml = self
                    .nons
                    .values()
                    .map(|n| n.serialize_yaml(self.flat))
                    .collect::<Vec<_>>()
                    .join("\n");
                result.push_str(&yaml);
            }
            OutputFormat::Non => {
                let nons = self
                    .nons
                    .values()
                    .map(|n| n.serialize_non(self.flat))
                    .collect::<Vec<_>>()
                    .join("\n");
                result.push_str(&nons);
            }
        }

        result
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &'a Non<'a>)> {
        self.nons.iter().map(|(k, &v)| (k, v))
    }

    pub fn get_all(&self) -> &HashMap<String, &'a Non<'a>> {
        &self.nons
    }
}

#[derive(Default)]
pub struct NonDefsBuilder<'a> {
    nons: Option<HashMap<String, &'a Non<'a>>>,
    format: Option<OutputFormat>,
    flat: Option<bool>,
}

impl<'a> NonDefsBuilder<'a> {
    pub fn build(self) -> NonDefs<'a> {
        NonDefs::new(
            self.nons.unwrap_or_default(),
            self.format.unwrap_or(OutputFormat::Json),
            self.flat.unwrap_or(false),
        )
    }

    pub fn nons(mut self, nons: HashMap<String, &'a Non<'a>>) -> Self {
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
