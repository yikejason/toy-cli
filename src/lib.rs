mod cli;
mod process;
mod utils;

pub use cli::{
    parse_format_base64, Base64Format, Base64Subcommand, HttpSubcommand, Opts, OutputFormat,
    SubCommand, TextSignFormat, TextSubcommand,
};
pub use process::*;
pub use utils::*;

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
