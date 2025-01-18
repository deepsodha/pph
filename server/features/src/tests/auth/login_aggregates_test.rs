#[cfg(test)]
mod tests {
    use cosmo_store_util::aggregate::Aggregate;

    use auth::domain::{
        auth_aggregate::LOGIN_AGGREGATE,
        auth_commands::{LoginCommand, LoginWithMobileCommand},
    };

    #[test]
    fn login_should_be_created_for_valid_command() {
        let events = LOGIN_AGGREGATE
            .execute(
                &LOGIN_AGGREGATE.init(),
                &LoginCommand::LoginWithMobileCommand(LoginWithMobileCommand {
                    mobile: ("9876543210").to_string(),
                    password: ("1234567890").to_string(),
                }),
            )
            .unwrap();

        assert_eq!(events.len(), 1);

        let state = events
            .iter()
            .fold(LOGIN_AGGREGATE.init(), |a, b| LOGIN_AGGREGATE.apply(a, b))
            .unwrap();

        assert_eq!(state.mobile, "9876543210".to_string());
    }
}
