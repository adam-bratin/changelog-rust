use crate::{
    cli::{app::*, generate::*},
    error::ChangelogError,
    fs::*,
    git::*,
    json::{change_file::*, config::*, serializable::Serializable},
    NewResult,
};
use chrono::prelude::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::path::Path;

fn get_type(sections: &[String]) -> String {
    let mut result = String::new();
    let index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Changelog Type")
        .items(&sections)
        .interact()
        .ok();
    result = match index {
        Some(i) => sections.get(i).unwrap_or(&result).clone(),
        _ => result,
    };
    result
}

fn get_description(change_type: &str) -> String {
    let result = String::new();
    let description_prompt = match change_type {
        k if k == DefaultSections::Other.as_ref() => "Enter description of change:",
        _ => "Enter description of chage: \n - <JIRA_TAG> <description>",
    };
    let prompt: Option<String> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(description_prompt)
        .interact()
        .ok();
    match prompt {
        Some(d) => d.trim().to_string(),
        _ => result,
    }
}

pub async fn run(cli: &ApplicationArgs, cmd: &GenerateCommand) -> NewResult<()> {
    let config = Configuration::parse(&cli.config).await?;

    let change_type = match &cmd.kind {
        Some(t) => t.clone(),
        _ => get_type(&config.sections),
    };
    if !config.sections.iter().any(|s| s == &change_type) {
        return Err(ChangelogError::from("invalid change_type"));
    }
    let description = match &cmd.description {
        Some(d) => d.clone(),
        _ => get_description(&change_type),
    };
    if description.is_empty() {
        return Err(ChangelogError::from("description cannot be empty"));
    }
    let date = Utc::now();
    let filename = ChangeFile::new_filename(Some(date)).await?;
    let out_path = Path::new(&cmd.output).join(filename);
    let change = ChangeFile::new(date, get_author().await?, change_type.clone(), description);
    write_text_to_file(&out_path, &change.to_json()?).await
}
