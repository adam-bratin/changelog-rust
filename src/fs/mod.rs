extern crate path_absolutize;
extern crate tokio;
use crate::NewResult;
use std::path::*;

pub async fn read_text_from_file(input: &Path) -> NewResult<String> {
    let buf = tokio::fs::read(input).await?;
    let contents = String::from_utf8_lossy(&buf).parse()?;
    Ok(contents)
}

pub async fn write_text_to_file(input: &Path, data: &str) -> NewResult<()> {
    tokio::fs::write(input, data).await?;
    Ok(())
}
