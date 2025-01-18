use anyhow::{bail, Result};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct ValidMobile(String);

impl ValidMobile {
    pub fn parse(s: &str) -> Result<ValidMobile> {
        let phone_no = Regex::new(
            r"(?x)
          (?:\+?1)?                       
          [\s\.]?
          (([2-9]\d{2})|\(([2-9]\d{2})\)) 
          [\s\.\-]?
          ([2-9]\d{2})                    
          [\s\.\-]?
          (\d{4})                         ",
        )
        .unwrap();
        // TODO: add proper error using thisError
        if phone_no.is_match(s) {
            Ok(ValidMobile(s.to_string()))
        } else {
            bail!("Phone number not valid")
        }
    }
}

impl AsRef<str> for ValidMobile {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
