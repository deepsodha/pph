use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct RequiredRespiratoryRate(f32);
impl RequiredRespiratoryRate {
    pub fn parse(rr: f32) -> Result<RequiredRespiratoryRate> {
        // TODO: this will be change
        if rr < 18.0 && rr > 10.0 {
            Ok(RequiredRespiratoryRate(rr))
        } else {
            // TODO: add proper error using thisError
            bail!("invalid respiratory rate")
        }
    }
}

#[derive(Debug, Clone)]
pub struct RespiratoryRateInput {
    pub respiratory_rate: f32,
}

#[derive(Debug, Clone)]
pub struct RespiratoryRate {
    pub respiratory_rate: RequiredRespiratoryRate,
}

#[cfg(test)]
mod tests {
    use crate::domain::vital::types::respiratory_rate::RequiredRespiratoryRate;
    use quickcheck::{Arbitrary, Gen};
    use rand::Rng;
    use speculoos::prelude::*;

    #[derive(Debug, Clone)]
    struct RequiredRespiratoryRateFloat(f32);
    impl Arbitrary for RequiredRespiratoryRateFloat {
        fn arbitrary(_: &mut Gen) -> Self {
            let mut rng = rand::thread_rng();
            RequiredRespiratoryRateFloat(rng.gen_range(10.0..18.0))
        }
    }

    #[test]
    fn greater_then_18_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredRespiratoryRate::parse(rng.gen_range(18.0..10000.0));
        assert_that!(res).is_err();
    }

    #[test]
    fn less_then_10_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredRespiratoryRate::parse(rng.gen_range(0.0..10.0));
        assert_that!(res).is_err();
    }
}
