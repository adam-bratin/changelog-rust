use crate::NewResult;
use chrono::prelude::*;
const DATE_FMT: &str = "%m/%d/%Y";
pub fn date_to_str<Tz: TimeZone>(date: &DateTime<Tz>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    date.clone().format(DATE_FMT).to_string()
}

pub fn date_to_rfc<Tz: TimeZone>(date: &DateTime<Tz>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    date.to_rfc3339()
}

pub fn date_from_str(s: &String) -> NewResult<DateTime<Utc>> {
    let parsed = DateTime::parse_from_str(s, DATE_FMT)?;
    Ok(DateTime::from(parsed))
}

pub fn date_from_rfc(s: &String) -> NewResult<DateTime<Utc>> {
    let parsed = DateTime::parse_from_rfc3339(s)?;
    Ok(DateTime::from(parsed))
}
