use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct RequiredSpo2(f32);
impl RequiredSpo2 {
    pub fn parse(level: f32) -> Result<RequiredSpo2> {
        // TODO: this will be change
        if level < 105_f32 && level > 85_f32 {
            Ok(RequiredSpo2(level))
        } else {
            // TODO: add proper error using thisError
            bail!("invalid spo2")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Spo2Input {
    pub spo2: f32,
}

#[derive(Debug, Clone)]
pub struct Spo2 {
    pub spo2: RequiredSpo2,
}

#[cfg(test)]
mod tests {
    use crate::domain::vital::types::spo2::RequiredSpo2;
    use quickcheck::{Arbitrary, Gen};
    use rand::Rng;
    use speculoos::prelude::*;

    #[derive(Debug, Clone)]
    struct RequiredRespiratoryRateFloat(f32);
    impl Arbitrary for RequiredRespiratoryRateFloat {
        fn arbitrary(_: &mut Gen) -> Self {
            let mut rng = rand::thread_rng();
            RequiredRespiratoryRateFloat(rng.gen_range(85.0..105.0))
        }
    }

    #[test]
    fn greater_then_105_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredSpo2::parse(rng.gen_range(105.0..10000.0));
        assert_that!(res).is_err();
    }

    #[test]
    fn less_then_85_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredSpo2::parse(rng.gen_range(0.0..85.0));
        assert_that!(res).is_err();
    }
}
