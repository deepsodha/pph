use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct RequiredWeight(f32);
impl RequiredWeight {
    pub fn parse(w: f32) -> Result<RequiredWeight> {
        // TODO: this will be change
        if w < 700_f32 && w > 0_f32 {
            Ok(RequiredWeight(w))
        } else {
            // TODO: add proper error using thisError
            bail!("invalid weight")
        }
    }
}

#[derive(Debug, Clone)]
pub struct WeightInput {
    pub weight: f32,
}

#[derive(Debug, Clone)]
pub struct Weight {
    pub weight: RequiredWeight,
}

#[cfg(test)]
mod tests {
    use crate::domain::vital::types::weight::RequiredWeight;
    use quickcheck::{Arbitrary, Gen};
    use rand::Rng;
    use speculoos::prelude::*;

    #[derive(Debug, Clone)]
    struct RequiredWeightFloat(f32);
    impl Arbitrary for RequiredWeightFloat {
        fn arbitrary(_: &mut Gen) -> Self {
            let mut rng = rand::thread_rng();
            RequiredWeightFloat(rng.gen_range(0.0..700.0))
        }
    }

    #[test]
    fn greater_then_700_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredWeight::parse(rng.gen_range(700.0..10000.0));
        assert_that!(res).is_err();
    }

    #[test]
    fn less_then_0_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredWeight::parse(rng.gen_range(-100.0..0.00));
        assert_that!(res).is_err();
    }
}
