use anyhow::{bail, Result};
use validator::ValidateUrl;

#[derive(Debug, Clone)]
pub struct Url(String);
impl Url {
    pub fn parse(s: &str) -> Result<Url> {
        if s.validate_url() {
            Ok(Url(s.to_string()))
        } else {
            bail!(" url must be valid")
        }
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
