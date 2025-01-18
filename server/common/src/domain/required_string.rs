use anyhow::{bail, Ok, Result};
use validator::ValidateLength;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequiredString(pub String);
impl RequiredString {
    pub fn parse(s: &str) -> Result<RequiredString> {
        if s.trim().validate_length(Some(1_u64), None, None) {
            Ok(RequiredString(s.to_string()))
        } else {
            // TODO: add proper error using thisError
            bail!("string is required")
        }
    }
}

impl AsRef<str> for RequiredString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
