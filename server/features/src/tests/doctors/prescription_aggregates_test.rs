#[cfg(test)]
mod tests {
    use chrono::Utc;
    use cosmo_store_util::aggregate::Aggregate;
    use prescription::domain::{
        prescription_aggregate::PRESCRIPTION_AGGREGATE,
        prescription_commands::{CreatePrescription, PrescriptionCommand},
    };

    #[test]
    fn prescription_should_be_created_for_valid_command() {
        let events = PRESCRIPTION_AGGREGATE
            .execute(
                &PRESCRIPTION_AGGREGATE.init(),
                &PrescriptionCommand::CreatePrescription(CreatePrescription {
                    id: "1".to_string(),
                    org_id: "1".to_string(),
                    patient_id: "1".to_string(),
                    doctor_id: "1".to_string(),
                    patient_name: "hey".to_string(),
                    presecription_name: "hi".to_string(),
                    date: Utc::now(),
                    drug_name: vec!["hello".to_string()],
                    instruction: Some("1".to_string()),
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
            .fold(PRESCRIPTION_AGGREGATE.init(), |a, b| {
                PRESCRIPTION_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.org_id, "1".to_string());
    }

    #[test]
    fn prescription_should_be_updated_for_valid_command() {
        let events = PRESCRIPTION_AGGREGATE
            .execute(
                &PRESCRIPTION_AGGREGATE.init(),
                &PrescriptionCommand::CreatePrescription(CreatePrescription {
                    id: "1".to_string(),
                    org_id: "1".to_string(),
                    patient_id: "1".to_string(),
                    doctor_id: "1".to_string(),
                    patient_name: "hey".to_string(),
                    presecription_name: "hi".to_string(),
                    date: Utc::now(),
                    drug_name: vec!["hello".to_string()],
                    instruction: Some("1".to_string()),
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
            .fold(PRESCRIPTION_AGGREGATE.init(), |a, b| {
                PRESCRIPTION_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.org_id, "1".to_string());
    }
}
