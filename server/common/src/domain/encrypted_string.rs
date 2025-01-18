use anyhow::{bail, Result};
use utils::password_helper::{hash_password, verify_password};

use super::required_string::RequiredString;

#[derive(Debug, Clone)]
pub struct HashPasswordString(String);

impl HashPasswordString {
    pub fn parse(s: &RequiredString) -> Result<HashPasswordString> {
        let hash = hash_password(s.as_ref())?;
        Ok(HashPasswordString(hash))
    }
    pub fn verify(hash: &str, password: &str) -> Result<HashPasswordString> {
        let res = verify_password(hash, password)?;
        if res {
            Ok(HashPasswordString(hash.to_string()))
        } else {
            bail!("Password didn't match")
        }
    }
}

impl AsRef<str> for HashPasswordString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
