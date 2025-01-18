#[cfg(test)]
mod tests {

    use chrono::Utc;
    use cosmo_store_util::aggregate::Aggregate;
    use doctor::dto::doctor_type::DoctorType;
    use patient::domain::immunizations::addhistorical::{
        addhistorical_aggregate::HISTORICAL_AGGREGATE,
        addhistorical_commands::{CreateHistorical, HistoricalCommand},
    };

    #[test]
    fn historical_should_be_crated_for_valid_command() {
        let events = HISTORICAL_AGGREGATE
            .execute(
                &HISTORICAL_AGGREGATE.init(),
                &HistoricalCommand::CreateHistorical(CreateHistorical {
                    id: "1".to_string(),
                    org_id: "1".to_string(),
                    patient_id: "1".to_string(),
                    vaccine: "vaccine".to_string(),
                    types: "types".to_string(),
                    date: Utc::now(),
                    number_in_series: "deep".to_string(),
                    provider: DoctorType::Cardiologist,
                    source_of_information: "source_of_information".to_string(),
                    comments: "no comments".to_string(),
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
            .fold(HISTORICAL_AGGREGATE.init(), |a, b| {
                HISTORICAL_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.provider, DoctorType::Cardiologist);
    }
}
