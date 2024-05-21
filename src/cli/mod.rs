mod base64;
mod csv;
mod genpass;
mod text;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;

use self::{csv::CsvOps, genpass::GenPassOps};

pub use self::base64::Base64Format;
pub use self::base64::Base64Subcommand;
pub use self::csv::OutputFormat;

pub use self::text::{TextSignFormat, TextSignOps, TextSubCommand, TextVerifyOps};

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

    #[command(subcommand)]
    Text(TextSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    // if input is "-" or file exists
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File does not exist"));
    }
}
