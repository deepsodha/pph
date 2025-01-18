use anyhow::{bail, Result};
use regex::Regex;
use validator::ValidateEmail;

#[derive(Debug, Clone)]
pub struct ValidEmailOrMobile(String);

impl ValidEmailOrMobile {
    pub fn parse(s: &str) -> Result<ValidEmailOrMobile> {
        let phone_no_regex = Regex::new(
            r#"(?x)
          (?:\+?1)?                       # Country Code Optional
          [\s\.]?
          (([2-9]\d{2})|\(([2-9]\d{2})\)) # Area Code
          [\s\.\-]?
          ([2-9]\d{2})                    # Exchange Code
          [\s\.\-]?
          (\d{4})                         # Subscriber Number"#,
        )
        .unwrap();

        if s.validate_email() || phone_no_regex.is_match(s) {
            Ok(ValidEmailOrMobile(s.to_string()))
        } else {
            bail!("email or phone number not valid")
        }
    }
}

impl AsRef<str> for ValidEmailOrMobile {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
