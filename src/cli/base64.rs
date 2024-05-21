use super::verify_file;
use clap::Parser;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum Base64Subcommand {
    #[command(name = "decode", about = "base64 decode")]
    Decode(Base64DecodeOps),
    #[command(name = "encode", about = "base64 encode")]
    Encode(Base64EncodeOps),
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOps {
    #[arg(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser=parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOps {
    #[arg(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser, Copy, Clone)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    // match format {
    //     "standard" => Ok(Base64Format::Standard),
    //     "urlsafe" => Ok(Base64Format::UrlSafe),
    //     _ => Err("Invalid base64 format"),
    // }
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid base64 format")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
