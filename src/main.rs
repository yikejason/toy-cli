use anyhow::Result;
use clap::Parser;
use toy_cli::{process_csv, Opts, SubCommand};

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
    }

    Ok(())
}
