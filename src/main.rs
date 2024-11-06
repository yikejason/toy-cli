use anyhow::Result;
use clap::Parser;
use std::fs;
use toy_cli::{
    process_base64_decode, process_base64_encode, process_csv, process_genpass, process_http_serve,
    process_text_decrypt, process_text_encypt, process_text_generate, process_text_sign,
    process_text_verify, Base64Subcommand, HttpSubcommand, Opts, SubCommand, TextSignFormat,
    TextSubcommand,
};
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = match opts.output {
                Some(output) => output.clone(),
                None => format!("output.{}", opts.format),
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", password);

            let estimate = zxcvbn(&password, &[]);
            eprintln!("Strength: {}", estimate.score());
        }
        SubCommand::Base64(subcommand) => match subcommand {
            Base64Subcommand::Encode(opts) => {
                let encode = process_base64_encode(&opts.input, opts.format)?;
                println!("{}", encode);
            }
            Base64Subcommand::Decode(opts) => {
                let decode = process_base64_decode(&opts.input, opts.format)?;

                let decode = String::from_utf8(decode)?;
                println!("{:?}", decode);
            }
        },
        SubCommand::Text(subcommand) => match subcommand {
            TextSubcommand::Sign(opts) => {
                let sig = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", sig);
            }
            TextSubcommand::Verify(opts) => {
                let verified = process_text_verify(&opts.input, &opts.key, &opts.sig, opts.format)?;
                println!("{}", verified);
            }
            TextSubcommand::Generate(opts) => {
                let key = process_text_generate(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
            TextSubcommand::Encrypt(opts) => {
                let encrypted =
                    process_text_encypt(&opts.input, &opts.key, &opts.nonce, opts.format)?;
                println!("{}", encrypted)
            }
            TextSubcommand::Decrypt(opts) => {
                let decrypted =
                    process_text_decrypt(&opts.input, &opts.key, &opts.nonce, opts.format)?;
                println!("{:?}", decrypted)
            }
        },
        SubCommand::Http(subcommand) => match subcommand {
            HttpSubcommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
            }
        },
    }

    Ok(())
}
