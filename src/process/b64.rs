use crate::{get_reader, Base64Format};
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

pub fn process_base64_encode(input: &str, format: Base64Format) -> Result<String> {
    let mut reader = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };

    Ok(encode)
}

pub fn process_base64_decode(input: &str, format: Base64Format) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let buf = buf.trim();

    let decode = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    Ok(decode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_base64_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_base64_encode(input, format).is_ok());
    }
    #[test]
    fn test_process_base64_decode() {
        let input = "fixtures/hello_encode.txt";
        let format = Base64Format::UrlSafe;
        assert!(process_base64_decode(input, format).is_ok());
    }
}
