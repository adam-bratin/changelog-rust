use crate::date::*;
use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};
use std::sync::atomic::{AtomicBool, Ordering};

pub static IS_MERGE: AtomicBool = AtomicBool::new(false);

// The signature of a serialize_with function must follow the pattern:
//
//    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
//    where
//        S: Serializer
//
// although it may also be generic over the input types T.
pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let d_str = match IS_MERGE.load(Ordering::Relaxed) {
        true => date_to_str(date),
        false => date_to_rfc(date),
    };
    serializer.serialize_str(d_str.as_str())
}

// The signature of a deserialize_with function must follow the pattern:
//
//    fn deserialize<'de, D>(D) -> Result<T, D::Error>
//    where
//        D: Deserializer<'de>
//
// although it may also be generic over the output types T.
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match IS_MERGE.load(Ordering::Relaxed) {
        true => date_from_str(&s),
        false => date_from_rfc(&s),
    }
    .map_err(serde::de::Error::custom)
}
