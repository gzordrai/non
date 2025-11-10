use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "kebab_case")]
pub enum OutputFormat {
    Json,
    Yaml,
    Non,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub path: String,

    #[arg(short, long)]
    pub format: Option<OutputFormat>,

    #[arg(short, long)]
    pub output: Option<String>,
}
