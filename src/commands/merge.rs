use crate::{
    cli::{app::*, merge::*},
    fs::*,
    git::{add, commit},
    json::{change_file::*, config::*},
    NewResult,
};
use futures::stream::{self, StreamExt};
use glob::glob;
use maplit::{convert_args, hashmap};
use std::collections::HashMap;
extern crate mustache;
use chrono::prelude::*;

async fn filter_chages(
    files: &mut glob::Paths,
    confg: &Configuration,
) -> NewResult<Vec<ChangeFile>> {
    let labels = &confg.sections;
    Ok(stream::iter(files)
        .filter_map(|entry| async move {
            match entry.ok() {
                Some(path) => ChangeFile::load(&path, labels).await,
                _ => None,
            }
        })
        .collect::<Vec<ChangeFile>>()
        .await)
}

pub async fn run(cli: &ApplicationArgs, cmd: &MergeCommand) -> NewResult<()> {
    let config = Configuration::parse(&cli.config).await?;
    let glob_str = format!("{}*.json", cmd.input.as_path().display().to_string());
    let mut files = glob(glob_str.as_str())?;
    let changes = filter_chages(&mut files, &config).await?;
    let now = Local::now();
    let mut data: HashMap<String, String> = convert_args!(
        keys = String::from,
        hashmap!(
            "date" =>format!("{month}/{day}/{year}",month=now.month(),day=now.day(),year=now.year()),
            "versionNoV"=> cmd.version.clone(),
            "version"=> format!("v{}", cmd.version),
            "name"=> config.name.clone()
        )
    );
    changes.iter().for_each(|change| {
        let value = change.get_entry();
        if !data.contains_key(&change.label) {
            data.insert(change.label.clone(), value);
        } else {
            data.get_mut(&change.label)
                .unwrap()
                .push_str(value.as_str());
        }
    });
    let template = mustache::compile_path(&cmd.template)?;
    let changelog = template.render_to_string(&data)?;
    let res = write_text_to_file(&cmd.output, &changelog).await;
    if cmd.delete_changes {
        tokio::fs::remove_dir_all(&cmd.input).await?;
        add(&cmd.input).await?;
        commit(
            "- remove change files from release",
            &config.extra_commit_args,
        )
        .await
    } else {
        res
    }
}
