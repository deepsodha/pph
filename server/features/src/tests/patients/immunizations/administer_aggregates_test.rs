#[cfg(test)]
mod tests {

    use chrono::Utc;
    use cosmo_store_util::aggregate::Aggregate;
    use doctor::dto::doctor_type::DoctorType;
    use patient::domain::immunizations::administer::{
        administer_aggregate::ADMINISTER_AGGREGATE,
        administer_commands::{AdministerCommand, CreateAdminister, UpdateAdminister},
    };

    #[test]
    fn administer_should_be_crated_for_valid_command() {
        let events = ADMINISTER_AGGREGATE
            .execute(
                &ADMINISTER_AGGREGATE.init(),
                &AdministerCommand::CreateAdminister(CreateAdminister {
                    id: "1".to_string(),
                    org_id: "1".to_string(),
                    patient_id: "1".to_string(),
                    vaccine: "no comments".to_string(),
                    types: common::dto::types::Types::Type1,
                    brand: patient::dto::immunization::administer_types::Brand::Brand1,
                    ordered: chrono::Utc::now(),
                    recorded: chrono::Utc::now(),
                    dose: "no comments".to_string(),
                    site: "https://www.google.com".to_string(),
                    number_of_series: 5,
                    expiration: chrono::Utc::now(),
                    consent_obtain: "no comments".to_string(),
                    administrated_by: "no comments".to_string(),
                    clinic_location: "1".to_string(),
                    provider: DoctorType::Cardiologist,
                    vis_date: chrono::Utc::now(),
                    vfs_financial_class: "no comments".to_string(),
                    comments: "no comment".to_string(),
                    generic: "no comment".to_string(),
                    lot: 22,
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
            .fold(ADMINISTER_AGGREGATE.init(), |a, b| {
                ADMINISTER_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.comments, "no comment");
    }

    #[test]
    fn administer_should_be_updated_for_valid_command() {
        let events = ADMINISTER_AGGREGATE
            .execute(
                &ADMINISTER_AGGREGATE.init(),
                &AdministerCommand::UpdateAdminister(UpdateAdminister {
                    org_id: "1".to_string(),
                    patient_id: "1".to_string(),
                    vaccine: "no comments".to_string(),
                    types: common::dto::types::Types::Type1,
                    brand: patient::dto::immunization::administer_types::Brand::Brand1,
                    ordered: chrono::Utc::now(),
                    recorded: chrono::Utc::now(),
                    dose: "no comments".to_string(),
                    site: "https://www.google.com".to_string(),
                    number_of_series: 5,
                    expiration: chrono::Utc::now(),
                    consent_obtain: "no comments".to_string(),
                    administrated_by: "no comments".to_string(),
                    clinic_location: "1".to_string(),
                    provider: DoctorType::Cardiologist,
                    vis_date: chrono::Utc::now(),
                    vfs_financial_class: "no comments".to_string(),
                    comments: "no comment".to_string(),
                    id: "no comment".to_string(),
                    generic: "no comment".to_string(),
                    lot: 22,
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
            .fold(ADMINISTER_AGGREGATE.init(), |a, b| {
                ADMINISTER_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.comments, "no comment");
    }
}
