use super::required_string::RequiredString;
use crate::dto::gov_info::{GovIdType, GovInfoInput};
use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct GovNo(RequiredString);

impl GovNo {
    pub fn parse(s: &str, id_type: &GovIdType) -> Result<GovNo> {
        let res = RequiredString::parse(s)?;

        match id_type {
            GovIdType::AadhaarCard => {
                if res.0.len() != 12 {
                    bail!("Aadhaar card number must be 12 characters long");
                }
                if !res.0.chars().all(|c| c.is_numeric()) {
                    bail!("Aadhaar card number must contain only digits");
                }
            }
            GovIdType::DrivingLicense => {
                if res.0.len() < 5 {
                    bail!("Driving license number must be at least 5 characters long");
                }
                if !res.0.chars().all(|c| c.is_alphanumeric()) {
                    bail!("Driving license number must contain only alphanumeric characters");
                }
            }
            GovIdType::Passport => {
                if res.0.len() < 6 {
                    bail!("Passport number must be at least 6 characters long");
                }
                if !res.0.chars().all(|c| c.is_alphanumeric()) {
                    bail!("Passport number must contain only alphanumeric characters");
                }
            }
        }

        Ok(GovNo(res))
    }
}

impl AsRef<str> for GovNo {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct GovInfo {
    pub id_no: GovNo,
    pub id_type: GovIdType,
}

impl GovInfo {
    pub fn parse(input: &GovInfoInput) -> Result<GovInfo> {
        let res = GovInfo {
            id_type: input.id_type,
            id_no: GovNo::parse(&input.id_no, &input.id_type)?,
        };
        Ok(res)
    }
}

impl From<GovInfo> for GovInfoInput {
    fn from(s: GovInfo) -> Self {
        GovInfoInput {
            id_no: String::from(s.id_no.as_ref()),
            id_type: s.id_type,
        }
    }
}
