mod asset;
mod cli;
mod commands;
mod date;
mod error;
mod fs;
mod git;
mod json;

use cli::app::Command;
use commands::{generate, init, merge};
use structopt::*;

pub type NewResult<T> = Result<T, Box<dyn std::error::Error>>;
pub const IS_WINDOWS: bool = cfg!(target_os = "windows");
#[tokio::main]
async fn main() -> NewResult<()> {
    let opts = cli::app::ApplicationArgs::from_args();
    match &opts.command {
        Command::Generate(c) => generate::run(&opts, &c).await?,
        Command::Merge(c) => merge::run(&opts, &c).await?,
        Command::Init(c) => init::run(&opts, &c).await?,
    }
    Ok(())
}
