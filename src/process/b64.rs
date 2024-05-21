use anyhow::Result;
use std::io::Read;

use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::cli::Base64Format;

pub fn process_encode(reader: &mut dyn Read, format: Base64Format) -> Result<String> {
    /*
        if else 中 std::io::stdin()和 File::open(input)是两种不同的类型
        所以需要用Box<dyn Read>来包装
        这两个类型都实现了std::io::Read
    */

    let mut buf = Vec::new();
    // 读入到缓存buf中
    reader.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    // println!("encode: {}", encode);
    Ok(encode)
}

pub fn process_decode(reader: &mut dyn Read, format: Base64Format) -> Result<String> {
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
    Ok(decode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_reader;

    #[test]
    fn test_process_encode() -> Result<()> {
        let input = "Cargo.toml";
        let mut reader = get_reader(input)?;
        let format = Base64Format::Standard;
        assert!(process_encode(&mut reader, format).is_ok());
        Ok(())
    }

    #[test]
    fn test_process_decode() -> Result<()> {
        let input = "fixtures/b64.txt";
        let mut reader = get_reader(input)?;
        let format = Base64Format::UrlSafe;
        assert!(process_decode(&mut reader, format).is_ok());

        Ok(())
    }
}
