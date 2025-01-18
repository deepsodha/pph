use anyhow::{bail, Result};
use validator::ValidateEmail;

#[derive(Debug, Clone)]
pub struct ValidEmail(pub String);

impl ValidEmail {
    pub fn parse(s: &str) -> Result<ValidEmail> {
        // TODO: add proper error using thisError
        if s.validate_email() {
            Ok(ValidEmail(s.to_string()))
        } else {
            bail!("Email is not valid")
        }
    }
}

impl AsRef<str> for ValidEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
