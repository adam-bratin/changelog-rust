use crate::NewResult;
use serde::{de::DeserializeOwned, Serialize};
pub trait Serializable<T = Self>: Serialize + Sized + DeserializeOwned {
    fn to_json(&self) -> NewResult<String> {
        let s = serde_json::to_string_pretty(self)?;
        Ok(s)
    }
    fn from_json(s: &str) -> NewResult<Self> {
        let result: Self = serde_json::from_str(s)?;
        Ok(result)
    }
}
