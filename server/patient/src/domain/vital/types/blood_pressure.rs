use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct RequiredBloodPressure(u32);
impl RequiredBloodPressure {
    pub fn parse(bp: u32) -> Result<RequiredBloodPressure> {
        // TODO: this will be change
        if bp < 300 && bp > 80 {
            Ok(RequiredBloodPressure(bp))
        } else {
            // TODO: add proper error using thisError
            bail!("invalid blood pressure")
        }
    }
}

impl AsRef<u32> for RequiredBloodPressure {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct BloodPressureInput {
    pub blood_pressure: u32,
}

#[derive(Debug, Clone)]
pub struct BloodPressure {
    pub blood_pressure: RequiredBloodPressure,
}

#[cfg(test)]
mod tests {
    use crate::domain::vital::types::blood_pressure::RequiredBloodPressure;
    use quickcheck::{Arbitrary, Gen};
    use rand::Rng;
    use speculoos::prelude::*;

    #[derive(Debug, Clone)]
    struct BloodPressureInt(u32);
    impl Arbitrary for BloodPressureInt {
        fn arbitrary(_: &mut Gen) -> Self {
            let mut rng = rand::thread_rng();
            BloodPressureInt(rng.gen_range(80..300))
        }
    }

    #[test]
    fn greater_then_300_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredBloodPressure::parse(rng.gen_range(300..10000));
        assert_that!(res).is_err();
    }

    #[test]
    fn less_then_80_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredBloodPressure::parse(rng.gen_range(0..80));
        assert_that!(res).is_err();
    }

    #[test]
    fn blood_pressure_as_ref_test() {
        let input = RequiredBloodPressure::parse(90).unwrap();
        let res = input.as_ref();
        assert_eq!(res, &90);
    }
}
