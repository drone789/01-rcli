use std::{fmt, path::Path, str::FromStr};

use clap::Parser;

#[derive(Debug, Parser)]
// #[command(name="rcli", version, author, about,long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV , or Convert CSV to other formats")]
    Csv(CsvOps),
}
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}
#[derive(Debug, Parser)]
pub struct CsvOps {
    #[arg(short, long, default_value = "input.csv",value_parser = verify_file_exists)]
    pub input: String,

    #[arg(short, long)] // "output.json".into()
    pub output: Option<String>,

    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(long, default_value_t = true)]
    pub header: bool,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

fn verify_file_exists(file_name: &str) -> Result<String, String> {
    if Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err(format!("File {} does not exist", file_name))
    }
}

fn parse_format(format_str: &str) -> Result<OutputFormat, anyhow::Error> {
    // 调用了parse()方法，用于将字符串解析为指定的类型。在这里，它试图将format_str解析为OutputFormat类型
    // 告诉编译器要将字符串解析为OutputFormat类型的值
    format_str.parse::<OutputFormat>()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            // OutputFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            // "toml" => Ok(OutputFormat::Toml),
            _ => Err(anyhow::anyhow!("UnSupported format.")),
        }
    }
}
// into使用
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
