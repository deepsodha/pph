#[cfg(test)]
mod tests {
    use chrono::Utc;
    use cosmo_store_util::aggregate::Aggregate;
    use patient::domain::vital::{
        vital_aggregate::VITALS_AGGREGATE,
        vital_commands::{CreateVitals, UpdateVitals, VitalsCommand},
    };

    #[test]
    fn vital_should_be_created_for_valid_command() {
        let events = VITALS_AGGREGATE
            .execute(
                &VITALS_AGGREGATE.init(),
                &VitalsCommand::CreateVitals(CreateVitals {
                    id: "1".to_string(),
                    org_id: "1".to_string(),
                    patient_id: "1".to_string(),
                    doctor_id: "1".to_string(),
                    blood_pressure: Some(22),
                    heart_rate: Some(44),
                    comments: Some("drug name".to_string()),
                    height: Some(22),
                    weight: Some(44),
                    bmi: Some(44),
                    temperature: Some(44),
                    date_time: Some(Utc::now()),
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
            .fold(VITALS_AGGREGATE.init(), |a, b| VITALS_AGGREGATE.apply(a, b))
            .unwrap();

        assert_eq!(state.org_id, "1".to_string());
    }

    #[test]
    fn vital_should_be_updated_for_valid_command() {
        let events = VITALS_AGGREGATE
            .execute(
                &VITALS_AGGREGATE.init(),
                &VitalsCommand::UpdateVitals(UpdateVitals {
                    org_id: "1".to_string(),
                    patient_id: "1".to_string(),
                    doctor_id: "1".to_string(),
                    blood_pressure: Some(22),
                    heart_rate: Some(44),
                    comments: Some("44".to_string()),
                    height: Some(22),
                    weight: Some(44),
                    bmi: Some(44),
                    temperature: Some(44),
                    id: "1".to_string(),
                    date_time: Some(Utc::now()),
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
            .fold(VITALS_AGGREGATE.init(), |a, b| VITALS_AGGREGATE.apply(a, b))
            .unwrap();

        assert_eq!(state.org_id, "1".to_string());
    }
}
