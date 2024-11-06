use super::{verify_file, verify_path};
use crate::{
    parse_format_base64, process_text_decrypt, process_text_encypt, process_text_generate,
    process_text_sign, process_text_verify, Base64Format, CmdExecutor,
};
use clap::Parser;
use std::{fmt, path::PathBuf, str::FromStr};

#[derive(Debug, Parser)]
pub enum TextSubcommand {
    #[command(about = "sign a message with a private and return a signature ")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signature with a public key")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key")]
    Generate(TextKeyGenerateOpts),
    #[command(about = "encrypt a message")]
    Encrypt(TextEncryptOpts),
    #[command(about = "decrypt an encrypted message")]
    Decrypt(TextDecryptOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "_")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "_")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(long, short, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "_")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(short, long)]
    pub nonce: String,
    #[arg(long, default_value = "standard", value_parser = parse_format_base64)]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "_")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(short, long)]
    pub nonce: String,
    #[arg(long, default_value = "standard", value_parser = parse_format_base64)]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> &'static str {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            v => anyhow::bail!("invalid text format {}", v),
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let sig = process_text_sign(&self.input, &self.key, self.format)?;
        println!("{}", sig);
        Ok(())
    }
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let verified = process_text_verify(&self.input, &self.key, &self.sig, self.format)?;
        println!("{}", verified);
        Ok(())
    }
}

impl CmdExecutor for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = process_text_generate(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                tokio::fs::write(name, &key[0]).await?;
            }
            TextSignFormat::Ed25519 => {
                let name = self.output;
                tokio::fs::write(name.join("ed25519.sk"), &key[0]).await?;
                tokio::fs::write(name.join("ed25519.pk"), &key[1]).await?;
            }
        }
        Ok(())
    }
}

impl CmdExecutor for TextEncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encrypted = process_text_encypt(&self.input, &self.key, &self.nonce, self.format)?;
        println!("{}", encrypted);
        Ok(())
    }
}

impl CmdExecutor for TextDecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decrypted = process_text_decrypt(&self.input, &self.key, &self.nonce, self.format)?;
        println!("{}", decrypted);
        Ok(())
    }
}

impl CmdExecutor for TextSubcommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            TextSubcommand::Sign(opts) => opts.execute().await,
            TextSubcommand::Verify(opts) => opts.execute().await,
            TextSubcommand::Generate(opts) => opts.execute().await,
            TextSubcommand::Encrypt(opts) => opts.execute().await,
            TextSubcommand::Decrypt(opts) => opts.execute().await,
        }
    }
}
