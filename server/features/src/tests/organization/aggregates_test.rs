#[cfg(test)]
mod tests {
    use chrono::Utc;
    use common::dto::address::AddressInput;
    use cosmo_store_util::aggregate::Aggregate;
    use organization::domain::{
        organization_aggregate::ORGANIZATION_AGGREGATE,
        organization_commands::{CreateOrganization, OrganizationCommand, UpdateOrganization},
    };

    #[test]
    fn organization_should_be_created_for_valid_command() {
        let events = ORGANIZATION_AGGREGATE
            .execute(
                &ORGANIZATION_AGGREGATE.init(),
                &OrganizationCommand::CreateOrganization(CreateOrganization {
                    org_name: "ParthClinic".to_string(),
                    id: "123".to_string(),
                    details: "dffd".to_string(),
                    phone_number: "1234567890".to_string(),
                    email: "ParthClinic@gmail.com".to_string(),
                    address: AddressInput {
                        pin_code: "dffd".to_string(),
                        city: "dffd".to_string(),
                        state: "dffd".to_string(),
                        address_line: "dffd".to_string(),
                        country: "dffd".to_string(),
                    },
                    created_by: "deep".to_string(),
                    updated_by: "deep".to_string(),
                    created_at: Utc::now(),
                    last_updated: Utc::now(),
                }),
            )
            .unwrap();

        assert_eq!(events.len(), 1);

        let state = events
            .iter()
            .fold(ORGANIZATION_AGGREGATE.init(), |a, b| {
                ORGANIZATION_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.name, "ParthClinic".to_string());
    }

    #[test]
    fn organization_should_be_updated_for_valid_command() {
        let events = ORGANIZATION_AGGREGATE
            .execute(
                &ORGANIZATION_AGGREGATE.init(),
                &OrganizationCommand::UpdateOrganization(UpdateOrganization {
                    id: "1".to_string(),
                    org_name: "ParthClinic".to_string(),
                    details: "dffd".to_string(),
                    phone_number: "1234567890".to_string(),
                    email: "ParthClinic@gmail.com".to_string(),
                    address: AddressInput {
                        pin_code: "dffd".to_string(),
                        city: "dffd".to_string(),
                        state: "dffd".to_string(),
                        address_line: "dffd".to_string(),
                        country: "dffd".to_string(),
                    },
                    created_by: "deep".to_string(),
                    updated_by: "deep".to_string(),
                    created_at: Utc::now(),
                    last_updated: Utc::now(),
                }),
            )
            .unwrap();

        assert_eq!(events.len(), 1);

        let state = events
            .iter()
            .fold(ORGANIZATION_AGGREGATE.init(), |a, b| {
                ORGANIZATION_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.name, "ParthClinic".to_string());
    }
}
