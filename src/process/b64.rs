use crate::cli::Base64Format;
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::{fs::File, io::Read};

pub fn process_base64_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };

    println!("{}", encode);

    Ok(())
}

pub fn process_base64_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let buf = buf.trim();

    let decode = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    let decode = String::from_utf8(decode)?;

    println!("{}", decode);

    Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "_" {
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
    fn test_process_base64_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_base64_encode(input, format).is_ok());
    }
    #[test]
    fn test_process_base64_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::UrlSafe;
        assert!(process_base64_decode(input, format).is_ok());
    }
}
