use crate::{
    asset::Assets,
    cli::{app::ApplicationArgs, init::InitCommand},
    fs::*,
    json::{config::*, serializable::*},
    NewResult,
};
extern crate tokio;
use packer::Packer;

pub async fn run(cli: &ApplicationArgs, cmd: &InitCommand) -> NewResult<()> {
    let config = Configuration::new(&cmd.app_name);
    tokio::fs::create_dir_all(&cmd.input).await?;
    write_text_to_file(&cli.config, &config.to_json()?).await?;
    if let Some(t) = Assets::get_str("CHANGELOG.md.hbs") {
        write_text_to_file(&cmd.template, &String::from(t)).await?;
    }
    Ok(())
}
