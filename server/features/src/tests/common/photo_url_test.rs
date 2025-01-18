#[cfg(test)]
mod tests {
    use common::domain::photo_url::PhotoUrl;
    use fake::faker::lorem::raw::*;
    use fake::locales::EN;
    use fake::Fake;
    use quickcheck::{Arbitrary, Gen};
    use speculoos::prelude::*;

    #[derive(Debug, Clone)]
    struct URLString(String);
    impl Arbitrary for URLString {
        fn arbitrary(_: &mut Gen) -> Self {
            let res: String = "https://www.google.com".to_string();
            URLString(res)
        }
    }

    #[test]
    fn random_string_not_allowed() {
        let par: String = Paragraph(EN, 3..5).fake();
        let res = PhotoUrl::parse(&par);
        assert_that!(res).is_err();
    }
}
