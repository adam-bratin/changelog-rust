use std::boxed::Box;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
#[derive(Debug, Clone)]
pub struct ChangelogError {
    pub error_message: String,
}

impl Display for ChangelogError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str(self.error_message.as_str())
    }
}

impl Error for ChangelogError {
    fn description(&self) -> &str {
        self.error_message.as_str()
    }
}

impl ChangelogError {
    pub fn new(message: impl Into<String>) -> Self {
        ChangelogError {
            error_message: message.into(),
        }
    }

    pub fn from(message: impl Into<String>) -> Box<ChangelogError> {
        Box::new(ChangelogError::new(message))
    }
}
