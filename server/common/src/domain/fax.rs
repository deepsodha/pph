use crate::common::required_string::RequiredString;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Fax(RequiredString);
impl Fax {
    pub fn parse(s: &str) -> Result<Fax> {
        // TODO: Should add other fax related validations here
        Ok(Fax(RequiredString::parse(s)?))
    }
}

impl AsRef<str> for Fax {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
