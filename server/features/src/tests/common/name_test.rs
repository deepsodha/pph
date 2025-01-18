#[cfg(test)]
mod tests {
    use common::domain::name::Name;

    #[test]
    fn name_test() {
        let input = Name::parse("abc", "abc").unwrap();
        let res = input.as_ref();
        assert_eq!(res, "abc");
    }
}
