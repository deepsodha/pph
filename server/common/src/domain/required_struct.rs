use super::required_string::RequiredString;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Icd10(RequiredString);
impl Icd10 {
    pub fn parse(s: &str) -> Result<Icd10> {
        Ok(Icd10(RequiredString::parse(s)?))
    }
}

#[cfg(test)]
mod tests {

    use serde_json::json;
    use speculoos::prelude::*;

    use super::Icd10;
    #[test]
    pub fn isd_10_must_take_csv_input() {
        let data = json!(
              [
          {
            "0": "1",
            "A00": "A00",
            "A000": "A001",
            "Cholera due to Vibrio cholerae 01, biovar cholerae": "Cholera due to Vibrio cholerae 01, biovar eltor",
            "Cholera due to Vibrio cholerae 01, biovar cholerae__1": "Cholera due to Vibrio cholerae 01, biovar eltor",
            "Cholera": "Cholera"
          },
          {
            "0": "9",
            "A00": "A00",
            "A000": "A009",
            "Cholera due to Vibrio cholerae 01, biovar cholerae": "Cholera, unspecified",
            "Cholera due to Vibrio cholerae 01, biovar cholerae__1": "Cholera, unspecified",
            "Cholera": "Cholera"
          },
          {
            "0": "0",
            "A00": "A010",
            "A000": "A0100",
            "Cholera due to Vibrio cholerae 01, biovar cholerae": "Typhoid fever, unspecified",
            "Cholera due to Vibrio cholerae 01, biovar cholerae__1": "Typhoid fever, unspecified",
            "Cholera": "Typhoid fever"
          },
        ]);
        let abc = data.to_string();

        let valid_roles = Icd10::parse(&abc);
        assert_that!(valid_roles).is_ok();
    }
}
