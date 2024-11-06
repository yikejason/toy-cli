mod base64;
mod csv_opts;
mod genpass;
mod http;
mod text;

pub use self::{base64::*, csv_opts::*, genpass::*, http::*, text::*};

use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert to CSV")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64Subcommand),
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubcommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubcommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "_" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("file does not exist")
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(&path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("_"), Ok("_".into()));
        assert_eq!(verify_file("nonexistent_file"), Err("file does not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
    }
}
