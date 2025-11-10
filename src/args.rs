use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
#[clap(rename_all = "kebab_case")]
pub enum OutputFormat {
    Json,
    Yaml,
    Non,
}

impl ToString for OutputFormat {
    fn to_string(&self) -> String {
        match self {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Non => "non",
        }.to_string()
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

    #[arg(long, default_value_t=false)]
    pub flat: bool,
}
