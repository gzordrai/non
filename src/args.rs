use std::fmt::Display;

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
#[clap(rename_all = "kebab_case")]
pub enum OutputFormat {
    Json,
    Yaml,
    Non,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Json => f.write_str("json"),
            OutputFormat::Yaml => f.write_str("yaml"),
            OutputFormat::Non => f.write_str("non"),
        }
    }
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub path: String,

    #[arg(short, long, default_value_t = OutputFormat::Non)]
    pub format: OutputFormat,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, default_value_t = false)]
    pub flat: bool,
}
