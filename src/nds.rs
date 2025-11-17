use std::{collections::HashMap, ops::Index, str::FromStr};

use crate::{
    args::OutputFormat,
    error::{NonError, Result},
    lexer::NonLexer,
    non::Non,
    parser::NonParser,
};

pub struct NonDefs {
    nons: HashMap<String, Non>,
}

impl NonDefs {
    pub fn new(nons: HashMap<String, Non>) -> Self {
        NonDefs { nons }
    }

    pub fn at(&self, id: &str) -> Option<&Non> {
        self.nons.get(id)
    }

    pub fn resolve(&self, id: &str) -> Result<Non> {
        let non = self
            .nons
            .get(id)
            .ok_or_else(|| NonError::UndefinedNon(id.to_string()))?;

        self.resolve_non(non)
    }

    fn resolve_non(&self, non: &Non) -> Result<Non> {
        let mut resolved = Non {
            name: non.name.clone(),
            parents: vec![],
            fields: vec![],
        };

        for parent_name in &non.parents {
            let parent_non = self
                .nons
                .get(parent_name)
                .ok_or_else(|| NonError::UndefinedNon(parent_name.clone()))?;

            let resolved_parent = self.resolve_non(parent_non)?;

            resolved.merge_fields(&resolved_parent)?;
        }

        resolved.merge_fields(non)?;

        Ok(resolved)
    }

    // pub fn serialize(&self, format: OutputFormat, flat: bool) -> String {
    //     let mut str = String::new();

    //     match format {
    //         OutputFormat::Json => {
    //             str.push_str("[\n");
    //             let nons = self
    //                 .nons
    //                 .values()
    //                 .map(|n| n.serialize_json(flat))
    //                 .collect::<Vec<_>>()
    //                 .join(",\n");
    //             str.push_str(&nons);
    //             str.push_str("\n]");
    //         }
    //         OutputFormat::Yaml => {
    //             let yaml = self
    //                 .nons
    //                 .values()
    //                 .map(|n| n.serialize_yaml(flat))
    //                 .collect::<Vec<_>>()
    //                 .join("\n");
    //             str.push_str(&yaml);
    //         }
    //         OutputFormat::Non => {
    //             let nons = self
    //                 .nons
    //                 .values()
    //                 .map(|n| n.serialize_non(flat))
    //                 .collect::<Vec<_>>()
    //                 .join("\n");
    //             str.push_str(&nons);
    //         }
    //     }
    //     str
    // }
}

impl FromStr for NonDefs {
    type Err = NonError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lexer = NonLexer::new(s);
        let mut parser = NonParser::new(lexer);

        Ok(parser.parse()?)
    }
}

impl Index<&str> for NonDefs {
    type Output = Non;

    fn index(&self, index: &str) -> &Self::Output {
        self.nons.get(index).expect("Non not found")
    }
}
