use std::path::*;
use structopt::*;

#[derive(Debug, StructOpt)]
pub struct MergeCommand {
    #[structopt(
        short = "o",
        help = "path to output changelog",
        default_value = "./CHANGELOG.md",
        parse(from_os_str)
    )]
    pub output: PathBuf,

    #[structopt(
        short = "i",
        help = "path to input change files",
        default_value = "./changes/",
        parse(from_os_str)
    )]
    pub input: PathBuf,

    #[structopt(
        short = "t",
        help = "path to changelog template file",
        default_value = "./CHANGELOG.md.hbs",
        parse(from_os_str)
    )]
    pub template: PathBuf,

    #[structopt(
        short = "v",
        env = "VERSION",
        help = "version of current release",
        default_value = "0.0.1"
    )]
    pub version: String,

    #[structopt(
        short = "d",
        long = "delete",
        help = "whether to delete change files after changelog is created"
    )]
    pub delete_changes: bool,
}
