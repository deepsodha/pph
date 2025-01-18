use crate::common::required_string::RequiredString;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Zip(RequiredString);
impl Zip {
    pub fn parse(s: &str) -> Result<Zip> {
        Ok(Zip(RequiredString::parse(s)?))
    }
}

impl AsRef<str> for Zip {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

