#[cfg(test)]
mod tests {
    use fake::faker::lorem::raw::*;
    use fake::locales::EN;
    use fake::Fake;
    use quickcheck::{Arbitrary, Gen};
    use speculoos::prelude::*;

    use common::domain::min_max_words::RequiredMax300Words;

    #[derive(Debug, Clone)]
    struct WordsString(String);
    impl Arbitrary for WordsString {
        fn arbitrary(_: &mut Gen) -> Self {
            let res = Paragraph(EN, 3..5).fake();
            WordsString(res)
        }
    }

    #[test]
    fn string_not_allowed_then_300_words() {
        let par: String = Paragraph(EN, 300..500).fake();
        let res = RequiredMax300Words::parse(&par);
        assert_that!(res).is_err();
    }

    #[test]
    fn ref_test() {
        let input = RequiredMax300Words::parse("abc").unwrap();
        let res = input.as_ref();
        assert_eq!(res, "abc");
    }
}
