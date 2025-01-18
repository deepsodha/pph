use anyhow::{bail, Result};
use validator::ValidateUrl;

#[derive(Debug, Clone)]
pub struct PhotoUrl(String);
impl PhotoUrl {
    pub fn parse(s: &str) -> Result<PhotoUrl> {
        if s.validate_url() {
            Ok(PhotoUrl(s.to_string()))
        } else {
            bail!("Photo url must be valid")
        }
    }
}

impl AsRef<str> for PhotoUrl {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
