#[cfg(test)]
mod tests {
    use fake::faker::internet::raw::SafeEmail;
    use fake::locales::EN;
    use fake::Fake;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;
    use speculoos::prelude::*;

    use common::domain::email::ValidEmail;

    #[derive(Debug, Clone)]
    struct EmailString(String);
    impl Arbitrary for EmailString {
        fn arbitrary(_: &mut Gen) -> Self {
            let res = SafeEmail(EN).fake();
            EmailString(res)
        }
    }

    #[test]
    fn blank_string_not_allowed() {
        let res = ValidEmail::parse("");
        assert_that!(res).is_err();
    }

    #[test]
    fn empty_string_not_allowed() {
        let res = ValidEmail::parse(" ");
        assert_that!(res).is_err();
    }

    #[test]
    fn half_email_with_domain() {
        let res = ValidEmail::parse("@domain.com");
        assert_that!(res).is_err();
    }

    #[quickcheck]
    fn string_should_work_for_valid_string(non_empty: EmailString) {
        let res = ValidEmail::parse(&non_empty.0);
        assert_that!(res).is_ok();
    }

    #[test]
    fn as_ref_test() {
        let input = ValidEmail::parse("abc@abc.com").unwrap();
        let res = input.as_ref();
        assert_eq!(res, "abc@abc.com");
    }
}
