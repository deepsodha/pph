use anyhow::{anyhow, Result};

use super::required_string::RequiredString;

#[derive(Debug, Clone)]
pub struct Name(RequiredString);
impl Name {
    pub fn parse(s: &str, field_name: &str) -> Result<Name> {
        let res = RequiredString::parse(s)?;
        // Validate the name
        if !Self::is_valid_name(s) {
            return Err(anyhow!("{} name is not valid", field_name));
        }
        Ok(Name(res))
    }

    pub fn is_valid_name(s: &str) -> bool {
        !s.is_empty()
            && !s.chars().any(|c| c.is_ascii_digit())
            && s.chars().all(|c| c.is_alphabetic() || c == ' ' || c == '-')
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
