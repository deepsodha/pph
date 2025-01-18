use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct RequiredHeight(f32);
impl RequiredHeight {
    pub fn parse(h: f32) -> Result<RequiredHeight> {
        // TODO: this will be change
        if h < 10_f32 && h > 0_f32 {
            Ok(RequiredHeight(h))
        } else {
            // TODO: add proper error using thisError
            bail!("invalid height")
        }
    }
}

#[derive(Debug, Clone)]
pub struct HeightInput {
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct Height {
    pub height: RequiredHeight,
}

#[cfg(test)]
mod tests {
    use crate::domain::vital::types::height::RequiredHeight;
    use quickcheck::{Arbitrary, Gen};
    use rand::Rng;
    use speculoos::prelude::*;

    #[derive(Debug, Clone)]
    struct HeightFloat(f32);
    impl Arbitrary for HeightFloat {
        fn arbitrary(_: &mut Gen) -> Self {
            let mut rng = rand::thread_rng();
            HeightFloat(rng.gen_range(0.0..10.0))
        }
    }

    #[test]
    fn greater_then_10_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredHeight::parse(rng.gen_range(10.0..10000.0));
        assert_that!(res).is_err();
    }

    #[test]
    fn less_then_0_not_allowed() {
        let mut rng = rand::thread_rng();
        let res = RequiredHeight::parse(rng.gen_range(-100.0..0.0));
        assert_that!(res).is_err();
    }
}
