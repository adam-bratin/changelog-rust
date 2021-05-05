extern crate mustache;
use crate::{
    cli::{app::*, merge::*},
    date::date_to_str,
    fs::*,
    git::{add, commit},
    json::{change_file::*, config::*, my_date_format::IS_MERGE},
    NewResult,
};
use chrono::prelude::*;
use futures::stream::{self, StreamExt};
use glob::glob;
use mustache::MapBuilder;
use std::collections::HashMap;
use std::sync::atomic::Ordering;

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
    let mut template_data = MapBuilder::new()
        .insert_str("date", &date_to_str(&Local::now()))
        .insert_str("version", &format!("v{}", cmd.version))
        .insert_str("versionNoV", &cmd.version)
        .insert_str("name", config.name.clone());
    let mut sections = HashMap::new();
    changes.iter().for_each(|change| {
        if ChangeFile::is_valid(change, &config.sections) {
            let section = sections.entry(change.label.clone()).or_insert(vec![]);
            section.push(change);
        }
    });
    IS_MERGE.store(true, Ordering::Relaxed);
    template_data = sections.iter().fold(template_data, |data, (key, value)| {
        data.insert(key, &value)
            .expect(format!("failed to encode section: {}", key).as_str())
    });

    let template = mustache::compile_path(&cmd.template)?;
    let changelog = template.render_data_to_string(&template_data.build())?;
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
