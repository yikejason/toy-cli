use crate::{process_jwt_sign, process_jwt_verify, CmdExecutor};

use clap::Parser;
use enum_dispatch::enum_dispatch;
use time::{Duration, OffsetDateTime};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum JwtSubcommand {
    #[command(about = "sign a message and generate a jwt")]
    Sign(JwtSignOpts),
    #[command(about = "verify a jwt ")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(short, long)]
    pub sub: String,
    #[arg(short, long)]
    pub aud: String,
    #[arg(short, long, value_parser = parse_offset_date_time, default_value = "2d" )]
    pub exp: OffsetDateTime,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long)]
    pub token: String,
    #[arg(long)]
    pub aud: String,
    #[arg(long)]
    pub sub: String,
}

fn parse_offset_date_time(s: &str) -> Result<OffsetDateTime, &'static str> {
    let single = &s[s.len() - 1..];
    let t: i64 = s[..s.len() - 1].parse().map_err(|_| "Invalid duration")?;
    let dur = match single {
        "m" => Duration::minutes(t),
        "h" => Duration::hours(t),
        "d" => Duration::days(t),
        _ => return Err("Invalid duration unit"),
    };
    Ok(OffsetDateTime::now_utc() + dur)
}

impl CmdExecutor for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = process_jwt_sign(self.sub, self.aud, self.exp.unix_timestamp())?;
        println!("Generated JWT: {}", token);
        Ok(())
    }
}

impl CmdExecutor for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_jwt_verify(&self.token, &self.aud, &self.sub)?;
        Ok(())
    }
}
