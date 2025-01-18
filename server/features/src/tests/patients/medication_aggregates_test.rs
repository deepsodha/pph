#[cfg(test)]
mod tests {
    use chrono::Utc;
    use cosmo_store_util::aggregate::Aggregate;
    use patient::domain::medication::{
        medication_aggregate::MEDICATIONS_AGGREGATE,
        medication_commands::{CreateMedication, MedicationsCommand, UpdateMedication},
    };

    #[test]
    fn medication_should_be_created_for_valid_command() {
        let events = MEDICATIONS_AGGREGATE
            .execute(
                &MEDICATIONS_AGGREGATE.init(),
                &MedicationsCommand::CreateMedication(CreateMedication {
                    id: "1".to_string(),
                    org_id: "1".to_string(),
                    patient_id: "1".to_string(),
                    status: common::dto::status::Status::Active,
                    drug: "drug name".to_string(),
                    instruction: Some("some instruction".to_string()),
                    comments: "comments".to_string(),
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
            .fold(MEDICATIONS_AGGREGATE.init(), |a, b| {
                MEDICATIONS_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.org_id, "1");
    }

    #[test]
    fn medication_should_be_updated_for_valid_command() {
        let events = MEDICATIONS_AGGREGATE
            .execute(
                &MEDICATIONS_AGGREGATE.init(),
                &MedicationsCommand::UpdateMedication(UpdateMedication {
                    org_id: "1".to_string(),
                    patient_id: "1".to_string(),
                    status: common::dto::status::Status::Active,
                    drug: "drug name".to_string(),
                    instruction: Some("some instruction".to_string()),
                    comments: "comments".to_string(),
                    id: "1".to_string(),
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
            .fold(MEDICATIONS_AGGREGATE.init(), |a, b| {
                MEDICATIONS_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.org_id, "1");
    }
}
