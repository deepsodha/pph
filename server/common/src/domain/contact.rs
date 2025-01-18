use anyhow::{bail, Result};

use regex::Regex;

use crate::dto::contact::{ContactInput, PhoneNoTypeForContact};

use super::required_string::RequiredString;

#[derive(Debug, Clone)]
pub struct PhoneNo(RequiredString);
impl PhoneNo {
    pub fn parse(s: &str) -> Result<PhoneNo> {
        let res = RequiredString::parse(s)?;
        let phone_no = Regex::new(
            r#"(?x)
              (?:\+?1)?                       # Country Code Optional
              [\s\.]?
              (([2-9]\d{2})|\(([2-9]\d{2})\))? # Area Code
              [\s\.\-]?
              ([2-9]\d{2})                    # Exchange Code
              [\s\.\-]?
              (\d{4})                         # Subscriber Number"#,
        )
        .unwrap();
        if phone_no.is_match(s) {
            Ok(PhoneNo(res))
        } else {
            bail!("invalid phone number");
        }
    }
}

impl AsRef<str> for PhoneNo {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct Contact {
    pub number: PhoneNo,
    pub number_type: PhoneNoTypeForContact,
}

impl Contact {
    pub fn parse(input: &ContactInput) -> Result<Contact> {
        let res = Contact {
            number: PhoneNo::parse(&input.number)?,
            number_type: input.number_type,
        };
        Ok(res)
    }
}

impl From<Contact> for ContactInput {
    fn from(s: Contact) -> Self {
        ContactInput {
            number: String::from(s.number.as_ref()),
            number_type: s.number_type,
        }
    }
}
