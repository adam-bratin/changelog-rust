use crate::{fs::*, json::serializable::Serializable, NewResult};
extern crate serde_json;
use serde::{Deserialize, Serialize};
use std::path::*;
use strum::VariantNames;
use strum_macros::{AsRefStr, EnumString, EnumVariantNames};

#[derive(Debug, EnumString, AsRefStr, EnumVariantNames)]
#[strum(serialize_all = "PascalCase")]
pub enum DefaultSections {
    Feature,
    BugFix,
    Other,
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub name: String,
    pub extra_commit_args: Vec<String>,
    pub sections: Vec<String>,
}

impl Serializable for Configuration {}

impl Configuration {
    pub fn new(name: &str) -> Configuration {
        Configuration {
            name: String::from(name),
            extra_commit_args: vec![],
            sections: DefaultSections::VARIANTS
                .iter()
                .map(|section| String::from(*section))
                .collect(),
        }
    }

    pub async fn parse(input: &Path) -> NewResult<Configuration> {
        let json = read_text_from_file(input).await?;
        Configuration::from_json(&json)
    }
}
