use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct ValidPassword(pub String);

impl ValidPassword {
    pub fn parse(password: &str) -> Result<ValidPassword> {
        let contains_number = password.chars().any(|c| c.is_numeric());
        let contains_alphabet = password.chars().any(|c| c.is_alphabetic());
        let contains_special_char = password.chars().any(|c| !c.is_alphanumeric());
        let is_long_enough = password.len() >= 6;

        if contains_number && contains_alphabet && contains_special_char && is_long_enough {
            Ok(ValidPassword(password.to_string()))
        } else {
            bail!("Password is not valid")
        }
    }
}

impl AsRef<str> for ValidPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
