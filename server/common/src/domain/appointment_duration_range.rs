use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct AppointmentDuration(u32);

impl AppointmentDuration {
    pub fn parse(cm: &u32) -> Result<AppointmentDuration> {
        // Dereference cm to get the u32 value
        if *cm >= 10 && *cm <= 60 {
            let res = AppointmentDuration(*cm);
            Ok(res)
        } else {
            // TODO: add proper error using thisError
            bail!("invalid Duration")
        }
    }
}

impl AsRef<u32> for AppointmentDuration {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_duration() {
        // Test case for a valid duration (within range)
        let duration = AppointmentDuration::parse(&30);
        assert!(duration.is_ok());
    }

    #[test]
    fn invalid_duration() {
        // Test case for an invalid duration (out of range)
        let duration = AppointmentDuration::parse(&5);
        assert!(duration.is_err());
    }
}
