use clap::Parser;
use std::str::FromStr;
use std::{fmt, path::PathBuf};

use super::{verify_file, verify_path};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(name = "verify", about = "Verify a signed message")]
    Verify(TextVerifyOps),
    #[command(name = "sign", about = "Sign a message with a private key")]
    Sign(TextSignOps),
    #[command(about = "Generate a random blake3 key or ed25519 key pair")]
    Generate(KeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextVerifyOps {
    #[arg(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser=verify_file)]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(long, default_value = "blake3", value_parser=parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextSignOps {
    #[arg(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser=verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser=parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct KeyGenerateOpts {
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Parser, Copy, Clone)]
pub enum TextSignFormat {
    Blake3,
    // Sha256,
    Ed25519,
}

fn parse_text_sign_format(str_format: &str) -> Result<TextSignFormat, anyhow::Error> {
    /*
    尝试将字符串类型的format转换为TextSignFormat枚举类型
    由于TextSignFormat并没有默认实现FromStr trait，因此编译器无法自动完成这种转换，
    需要手动为TextSignFormat实现FromStr trait来定义这种转换规则。
     */
    str_format.parse()
}

// the trait bound `TextSignFormat: FromStr` is not satisfied
// the following other types implement trait `FromStr`....
/*
将字符串类型转换为TextSignFormat枚举类型的值
通过实现FromStr trait，可以定义从字符串到TextSignFormat枚举类型的转换规则，
使得可以从用户输入的字符串中解析出相应的枚举值。

*/
impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("UnSupported format.")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
