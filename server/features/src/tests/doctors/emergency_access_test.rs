#[cfg(test)]
mod tests {
    use common::domain::emergency_access::EmergencyAccess;

    #[test]
    fn emergency_test() {
        let res = EmergencyAccess::parse(&true);
        assert!(res.is_ok());
    }
}
