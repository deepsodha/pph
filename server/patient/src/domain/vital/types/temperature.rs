use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct RequiredTemperature(f32);
impl RequiredTemperature {
    pub fn parse(t: f32) -> Result<RequiredTemperature> {
        // TODO: this will be change
        if t < 110_f32 && t > 90_f32 {
            Ok(RequiredTemperature(t))
        } else {
            // TODO: add proper error using thisError
            bail!("invalid temperature")
        }
    }
}

#[derive(Debug, Clone)]
pub struct TemperatureInput {
    pub temperature: f32,
}

#[derive(Debug, Clone)]
pub struct Temperature {
    pub temperature: RequiredTemperature,
}

#[cfg(test)]
mod tests {
    use crate::domain::vital::types::temperature::RequiredTemperature;
    use quickcheck::{Arbitrary, Gen};
    use rand::Rng;
    use speculoos::prelude::*;

    #[derive(Debug, Clone)]
    struct RequiredTemperatureFloat(f32);
    impl Arbitrary for RequiredTemperatureFloat {
        fn arbitrary(_: &mut Gen) -> Self {
            let mut rng = rand::thread_rng();
            RequiredTemperatureFloat(rng.gen_range(90.0..110.0))
        }
    }

    #[test]
    fn greater_then_110_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredTemperature::parse(rng.gen_range(110.0..10000.0));
        assert_that!(res).is_err();
    }

    #[test]
    fn less_then_90_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredTemperature::parse(rng.gen_range(0.0..90.0));
        assert_that!(res).is_err();
    }
}
