#[cfg(test)]
mod tests {
    use fake::faker::lorem::raw::*;
    use fake::locales::EN;
    use fake::Fake;
    use quickcheck::{Arbitrary, Gen};
    use speculoos::prelude::*;

    use common::domain::contact::PhoneNo;

    #[derive(Debug, Clone)]
    struct URLString(String);
    impl Arbitrary for URLString {
        fn arbitrary(_: &mut Gen) -> Self {
            let res: String = "+919876543210".to_string();
            URLString(res)
        }
    }

    #[test]
    fn random_string_not_allowed() {
        let par: String = Paragraph(EN, 3..5).fake();
        let res = PhoneNo::parse(&par);
        assert_that!(res).is_err();
    }

    #[test]
    fn blank_phone_number_not_allowed() {
        let res = PhoneNo::parse("");
        assert_that!(res).is_err();
    }

    #[test]
    fn empty_phone_number_not_allowed() {
        let res = PhoneNo::parse(" ");
        assert_that!(res).is_err();
    }

    #[test]
    fn half_phone_number_with_domain() {
        let res = PhoneNo::parse("98765");
        assert_that!(res).is_err();
    }

    #[test]
    fn phone_number_should_work_for_valid_phone_number() {
        let res = PhoneNo::parse("+919727057591");
        assert_that!(res).is_ok();
    }
}
