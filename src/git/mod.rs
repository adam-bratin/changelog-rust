use crate::{error::ChangelogError, NewResult};
use std::ffi::OsStr;
use std::path::Path;
use tokio::{join, process::*};

pub async fn run_git_command<I, S>(args: I) -> NewResult<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new("git").args(args).output().await?;
    let s = String::from_utf8(output.stdout)?;
    Ok(String::from(s.trim()))
}

pub async fn get_branch_name() -> NewResult<String> {
    let output = run_git_command(&["branch", "--show-current"]).await?;
    Ok(output)
}

pub async fn get_email() -> NewResult<String> {
    let result = run_git_command(&["config", "user.email"]).await?;
    Ok(result)
}

pub async fn get_name() -> NewResult<String> {
    let result = run_git_command(&["config", "user.name"]).await?;
    Ok(result)
}

pub async fn get_author() -> NewResult<String> {
    let (email_f, name_f) = join!(get_email(), get_name());
    let email = email_f?;
    let name = name_f?;
    if !email.is_empty() && !name.is_empty() {
        Ok(format!("{name} {email}", name = name, email = email))
    } else {
        Ok("Unkown user".to_string())
    }
}

fn check_git_output(output: &str, err_str: &str) -> NewResult<()> {
    if !output.contains("fatal") {
        Ok(())
    } else {
        Err(ChangelogError::from(err_str))
    }
}

pub async fn add(path: &Path) -> NewResult<()> {
    let result = run_git_command(&[OsStr::new("add"), path.as_os_str()]).await?;
    check_git_output(&result, "failed to add files to git")
}

pub async fn commit(message: impl Into<&str>, extra_args: &[String]) -> NewResult<()> {
    let mut args = vec!["commit", "-m", message.into()];
    extra_args.iter().for_each(|arg| args.push(arg.as_str()));
    let result = run_git_command(&args).await?;
    check_git_output(&result, "failed to commit")
}
