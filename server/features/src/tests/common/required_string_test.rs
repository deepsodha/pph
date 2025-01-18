#[cfg(test)]
mod tests {
    use common::domain::required_string::RequiredString;
    use fake::faker::lorem::raw::Word;
    use fake::locales::EN;
    use fake::Fake;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;
    use speculoos::prelude::*;

    #[derive(Debug, Clone)]
    struct NonEmptyString(String);
    impl Arbitrary for NonEmptyString {
        fn arbitrary(_: &mut Gen) -> Self {
            let res = Word(EN).fake();
            NonEmptyString(res)
        }
    }

    #[test]
    fn blank_string_not_allowed() {
        let res = RequiredString::parse("");
        assert_that!(res).is_err();
    }

    #[test]
    fn empty_string_not_allowed() {
        let res = RequiredString::parse(" ");
        assert_that!(res).is_err();
    }

    #[quickcheck]
    fn string_should_work_for_valid_string(non_empty: NonEmptyString) {
        let res = RequiredString::parse(&non_empty.0);
        assert_that!(res).is_ok();
    }
}
