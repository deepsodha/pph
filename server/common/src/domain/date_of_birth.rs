use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DateOfBirth(pub DateTime<Utc>);

impl DateOfBirth {
    pub fn parse(s: &DateTime<Utc>) -> Result<DateOfBirth> {
        if s < &Utc::now() {
            Ok(DateOfBirth(*s))
        } else {
            bail!("Date of birth must be in the past");
        }
    }
}

impl AsRef<DateTime<Utc>> for DateOfBirth {
    fn as_ref(&self) -> &DateTime<Utc> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeDelta;

    use super::*;

    #[test]
    fn valid_date_of_birth() {
        let days = match TimeDelta::try_days(1) {
            Some(delta) => delta,
            None => panic!("Failed to create TimeDelta for 1 day"),
        };
        let valid_date = Utc::now().checked_sub_signed(days).unwrap();
        assert!(DateOfBirth::parse(&valid_date).is_ok());
    }

    #[test]
    fn invalid_date_of_birth() {
        let invalid_dob = DateTime::parse_from_rfc3339("2050-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        assert!(DateOfBirth::parse(&invalid_dob).is_err());
    }
}
