#[cfg(test)]
mod tests {
    use common::domain::address::{AddressLine, City, Country, PinCode, State};

    #[test]
    fn pin_code_test() {
        let input = PinCode::parse("4567890").unwrap();
        let res = input.as_ref();
        assert_eq!(res, "4567890");
    }

    #[test]
    fn city_test() {
        let input = City::parse("abc").unwrap();
        let res = input.as_ref();
        assert_eq!(res, "abc");
    }

    #[test]
    fn state_test() {
        let input = State::parse("abc").unwrap();
        let res = input.as_ref();
        assert_eq!(res, "abc");
    }

    #[test]
    fn country_test() {
        let input = Country::parse("abc").unwrap();
        let res = input.as_ref();
        assert_eq!(res, "abc");
    }

    #[test]
    fn address_line_test() {
        let input = AddressLine::parse("abc").unwrap();
        let res = input.as_ref();
        assert_eq!(res, "abc");
    }
}
