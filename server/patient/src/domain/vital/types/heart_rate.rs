use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct RequiredHeartRate(u8);
impl RequiredHeartRate {
    pub fn parse(hr: u8) -> Result<RequiredHeartRate> {
        // TODO: this will be change
        if hr < 100 && hr > 55 {
            Ok(RequiredHeartRate(hr))
        } else {
            // TODO: add proper error using thisError
            bail!("invalid heart rate")
        }
    }
}

impl AsRef<u8> for RequiredHeartRate {
    fn as_ref(&self) -> &u8 {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct HeartRateInput {
    pub heart_rate: u8,
}

#[derive(Debug, Clone)]
pub struct HeartRate {
    pub heart_rate: RequiredHeartRate,
}

#[cfg(test)]
mod tests {
    use crate::domain::vital::types::heart_rate::RequiredHeartRate;
    use quickcheck::{Arbitrary, Gen};
    use rand::Rng;
    use speculoos::prelude::*;

    #[derive(Debug, Clone)]
    struct HeartRateInt(u8);
    impl Arbitrary for HeartRateInt {
        fn arbitrary(_: &mut Gen) -> Self {
            let mut rng = rand::thread_rng();
            HeartRateInt(rng.gen_range(55..100))
        }
    }

    #[test]
    fn greater_then_100_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredHeartRate::parse(rng.gen_range(100..255));
        assert_that!(res).is_err();
    }

    #[test]
    fn less_then_55_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredHeartRate::parse(rng.gen_range(0..55));
        assert_that!(res).is_err();
    }

    #[test]
    fn ref_test() {
        let input = RequiredHeartRate::parse(60).unwrap();
        let res = input.as_ref();
        assert_eq!(res, &60);
    }
}
