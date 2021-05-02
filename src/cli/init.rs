use std::path::PathBuf;
use structopt::*;

#[derive(Debug, StructOpt)]
pub struct InitCommand {
    #[structopt(
        short = "i",
        help = "path where change files will be located",
        default_value = "./changes/",
        parse(from_os_str)
    )]
    pub input: PathBuf,

    #[structopt(
        short = "t",
        help = "path to changelog template file to be generated",
        default_value = "./CHANGELOG.md.hbs",
        parse(from_os_str)
    )]
    pub template: PathBuf,

    #[structopt(short = "n", long = "appName", help = "name of app")]
    pub app_name: String,
}
