use std::{fmt, str::FromStr};

use crate::{process_base64_decode, process_base64_encode, CmdExecutor};

use super::verify_file;
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum Base64Subcommand {
    #[command(name = "encode", about = "encode a string to base64 string")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "decode a base64 string to string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "_")]
    pub input: String,
    #[arg(short, long, value_parser = parse_format_base64, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "_")]
    pub input: String,
    #[arg(short, long, value_parser = parse_format_base64, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

pub fn parse_format_base64(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> &'static str {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            v => anyhow::bail!("invalid base64 format {}", v),
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExecutor for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encode = process_base64_encode(&self.input, self.format)?;
        println!("{}", encode);
        Ok(())
    }
}

impl CmdExecutor for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decode = process_base64_decode(&self.input, self.format)?;
        let decode = String::from_utf8(decode)?;
        println!("{:?}", decode);
        Ok(())
    }
}
