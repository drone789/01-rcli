mod base64;
mod csv;
mod genpass;

use std::path::Path;

use clap::Parser;

use self::{csv::CsvOps, genpass::GenPassOps};

pub use self::base64::Base64Format;
pub use self::base64::Base64Subcommand;
pub use self::csv::OutputFormat;

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

    #[command(name = "genpass", about = "generate a random password")]
    GenPass(GenPassOps),

    #[command(subcommand)]
    Base64(Base64Subcommand),
}

fn verify_file_exists(file_name: &str) -> Result<String, &'static str> {
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file_exists() {
        assert_eq!(verify_file_exists("-"), Ok("-".into()));
        assert_eq!(verify_file_exists("not-exist"), Err("File does not exist"));
    }
}
