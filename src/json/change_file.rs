use super::{my_date_format, serializable::Serializable};
use crate::{fs::*, git::*, NewResult};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;
extern crate sanitize_filename;
use sanitize_filename::{sanitize_with_options, Options};

#[derive(Serialize, Deserialize, Clone)]
pub struct ChangeFile {
    #[serde(with = "my_date_format")]
    pub date: DateTime<Utc>,
    pub author: String,
    pub label: String,
    pub description: String,
}

impl Serializable for ChangeFile {}

impl ChangeFile {
    fn add_bullet(description: &str) -> String {
        if description.starts_with('-') {
            String::from(description)
        } else {
            format!("- {}", description)
        }
    }
    pub fn new(date: DateTime<Utc>, author: String, label: String, description: String) -> Self {
        ChangeFile {
            date,
            author,
            label,
            description: ChangeFile::add_bullet(&description),
        }
    }

    pub async fn new_filename(date: Option<DateTime<Utc>>) -> NewResult<String> {
        let _date = match date {
            Some(d) => d,
            _ => Utc::now(),
        };
        let branch_name = get_branch_name().await?;
        let filename = format!("{}-{}.json", branch_name, _date.format("%vT%T"));
        let options: Options = Options {
            truncate: true,
            windows: true,
            replacement: "_",
        };
        Ok(sanitize_with_options(filename, options))
    }

    async fn load_from_file(input: &Path) -> NewResult<ChangeFile> {
        let contents = read_text_from_file(input).await?;
        ChangeFile::from_json(&contents)
    }

    pub async fn load(input: &Path, labels: &[String]) -> Option<ChangeFile> {
        match ChangeFile::load_from_file(input).await.ok() {
            Some(change) => match ChangeFile::is_valid(&change, labels) {
                true => Some(change),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn get_entry(&self) -> String {
        format!(
            "{description} - {author}\n",
            description = self.description,
            author = self.author,
        )
    }

    pub fn is_valid(change: &ChangeFile, labels: &[String]) -> bool {
        labels.iter().any(|l| *l == change.label)
    }
}
