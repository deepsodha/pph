use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct RequiredBmi(f32);
impl RequiredBmi {
    pub fn parse(t: f32) -> Result<RequiredBmi> {
        // TODO: this will be change
        if t < 30_f32 && t > 15_f32 {
            Ok(RequiredBmi(t))
        } else {
            // TODO: add proper error using thisError
            bail!("invalid BMI")
        }
    }
}

impl AsRef<f32> for RequiredBmi {
    fn as_ref(&self) -> &f32 {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct BmiInput {
    pub bmi: f32,
}

#[derive(Debug, Clone)]
pub struct Bmi {
    pub bmi: RequiredBmi,
}

#[cfg(test)]
mod tests {
    use crate::domain::vital::types::bmi::RequiredBmi;
    use quickcheck::{Arbitrary, Gen};
    use rand::Rng;
    use speculoos::prelude::*;

    #[derive(Debug, Clone)]
    struct BmiFloat(f32);
    impl Arbitrary for BmiFloat {
        fn arbitrary(_: &mut Gen) -> Self {
            let mut rng = rand::thread_rng();
            BmiFloat(rng.gen_range(15.0..30.0))
        }
    }

    #[test]
    fn greater_then_30_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredBmi::parse(rng.gen_range(30.0..10000.0));
        assert_that!(res).is_err();
    }

    #[test]
    fn less_then_15_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredBmi::parse(rng.gen_range(0.0..15.0));
        assert_that!(res).is_err();
    }

    // #[test]
    // fn bmi_as_ref_test() {
    //     let input = RequiredBmi::parse(90.0).unwrap();
    //     let res = input.as_ref();
    //     assert_eq!(res, &90.0);
    // }
}
