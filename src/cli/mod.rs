mod base64;
mod csv_opts;
mod genpass;

use self::{csv_opts::CsvOpts, genpass::GenPassOpts};

pub use self::{
    base64::{Base64Format, Base64Subcommand},
    csv_opts::OutputFormat,
};

use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert to CSV")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64Subcommand),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "_" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("file does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("_"), Ok("_".into()));
        assert_eq!(
            verify_input_file("nonexistent_file"),
            Err("file does not exist")
        );
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
    }
}
