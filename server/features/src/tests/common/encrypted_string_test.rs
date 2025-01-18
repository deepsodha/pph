#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use common::domain::{encrypted_string::HashPasswordString, required_string::RequiredString};

    #[test]
    fn password_and_hash_are_not_same() {
        let password = "password";
        let required = RequiredString::parse(password).unwrap();
        let hash = HashPasswordString::parse(&required).unwrap();
        assert_ne!(password, hash.as_ref());
    }

    #[test]
    fn verify_password_should_verify_hash() {
        let password = "password";
        let required = RequiredString::parse(password).unwrap();
        let hash = HashPasswordString::parse(&required).unwrap();
        let verify = HashPasswordString::verify(hash.as_ref(), password);
        assert_ne!(hash.as_ref(), password);
        assert_that!(verify).is_ok();
    }
}
