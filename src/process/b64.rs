use anyhow::Result;
use std::fs::File;
use std::io::Read;

use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::cli::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    /*
        if else 中 std::io::stdin()和 File::open(input)是两种不同的类型
        所以需要用Box<dyn Read>来包装
        这两个类型都实现了std::io::Read
    */
    let mut reader = get_reader(input)?;

    let mut buf = Vec::new();
    // 读入到缓存buf中
    reader.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("{}", encode);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader: Box<dyn Read> = get_reader(input)?;

    let mut buf = String::new();
    // 读入到缓存buf中
    reader.read_to_string(&mut buf)?;
    // 去掉末尾的换行符
    let buf = buf.trim();

    let decode = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    // TODO: decode might not be a string.
    let decode = String::from_utf8(decode)?;
    println!("{}", decode);
    Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::UrlSafe;
        assert!(process_decode(input, format).is_ok());
    }
}
