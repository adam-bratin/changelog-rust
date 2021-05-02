use super::{generate::GenerateCommand, init::InitCommand, merge::MergeCommand};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    /// merges all the change files into an changelog from template
    #[structopt(name = "merge")]
    Merge(MergeCommand),

    /// generate chagefile for PR
    #[structopt(name = "generate")]
    Generate(GenerateCommand),

    /// initialize repo with setup for changelog cli
    #[structopt(name = "init")]
    Init(InitCommand),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "changelog")]
pub struct ApplicationArgs {
    #[structopt(
        short = "c",
        help = "path to config file",
        default_value = "./.changelogrc",
        parse(from_os_str)
    )]
    pub config: PathBuf,

    #[structopt(subcommand)]
    pub command: Command,
}
