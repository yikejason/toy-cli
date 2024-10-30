use anyhow::Result;
use clap::Parser;
use toy_cli::{
    process_base64_decode, process_base64_encode, process_csv, process_genpass, process_text_sign,
    process_text_verify, Base64Subcommand, Opts, SubCommand, TextSubcommand,
};

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = match opts.output {
                Some(output) => output.clone(),
                None => format!("output.{}", opts.format),
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => process_genpass(
            opts.length,
            opts.uppercase,
            opts.lowercase,
            opts.number,
            opts.symbol,
        )?,
        SubCommand::Base64(subcommand) => match subcommand {
            Base64Subcommand::Encode(opts) => {
                process_base64_encode(&opts.input, opts.format)?;
            }
            Base64Subcommand::Decode(opts) => {
                process_base64_decode(&opts.input, opts.format)?;
            }
        },
        SubCommand::Text(subcommand) => match subcommand {
            TextSubcommand::Sign(opts) => {
                process_text_sign(&opts.input, &opts.key, opts.format)?;
            }
            TextSubcommand::Verify(opts) => {
                process_text_verify(&opts.input, &opts.key, &opts.sig, opts.format)?;
            }
        },
    }

    Ok(())
}
