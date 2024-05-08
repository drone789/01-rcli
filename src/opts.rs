use std::path::Path;

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

#[derive(Debug, Parser)]
pub struct CsvOps {
    #[arg(short, long, default_value = "input.csv",value_parser = verify_file_exists)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    pub output: String,

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
