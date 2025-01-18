use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct RequiredHeadCirc(f32);
impl RequiredHeadCirc {
    pub fn parse(h: f32) -> Result<RequiredHeadCirc> {
        // TODO: this will be change
        if h > 10_f32 && h < 25_f32 {
            Ok(RequiredHeadCirc(h))
        } else {
            // TODO: add proper error using thisError
            bail!("invalid head circ")
        }
    }
}

impl AsRef<f32> for RequiredHeadCirc {
    fn as_ref(&self) -> &f32 {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct HeadCircInput {
    pub head_circ: f32,
}

#[derive(Debug, Clone)]
pub struct HeadCirc {
    pub head_circ: RequiredHeadCirc,
}

#[cfg(test)]
mod tests {
    use crate::domain::vital::types::head_circ::RequiredHeadCirc;
    use quickcheck::{Arbitrary, Gen};
    use rand::Rng;
    use speculoos::prelude::*;

    #[derive(Debug, Clone)]
    struct HeadCircFloat(f32);
    impl Arbitrary for HeadCircFloat {
        fn arbitrary(_: &mut Gen) -> Self {
            let mut rng = rand::thread_rng();
            HeadCircFloat(rng.gen_range(10.0..25.0))
        }
    }

    #[test]
    fn greater_then_25_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredHeadCirc::parse(rng.gen_range(25.0..10000.0));
        assert_that!(res).is_err();
    }

    #[test]
    fn less_then_10_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredHeadCirc::parse(rng.gen_range(0.0..10.0));
        assert_that!(res).is_err();
    }

    #[test]
    fn head_cric_ref_test() {
        let input = RequiredHeadCirc::parse(12.0).unwrap();
        let res = input.as_ref();
        assert_eq!(res, &12.0);
    }
}
