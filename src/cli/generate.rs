use std::path::*;
use structopt::*;
#[derive(Debug, StructOpt)]
pub struct GenerateCommand {
    #[structopt(
        short = "o",
        help = "path to output change file",
        default_value = "./changes/",
        parse(from_os_str)
    )]
    pub output: PathBuf,

    #[structopt(
        short = "d",
        help = "optional description for change (for automated generation)"
    )]
    pub description: Option<String>,

    #[structopt(
        short = "t",
        long = "type",
        help = "optional change type (for automated generation)"
    )]
    pub kind: Option<String>,
}
