#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, Utc};
    use common::dto::status::Status;
    use cosmo_store_util::aggregate::Aggregate;
    use patient::domain::allergies::{
        allergies_aggregate::ALLERGIES_AGGREGATE,
        allergies_commands::{AllergiesCommand, CreateAllergies, UpdateAllergies},
    };
    use patient::dto::allergies::allergy_severities::AllergySeveritiesType;

    #[test]
    fn allergies_should_be_crated_for_valid_command() {
        let input_date = NaiveDate::from_ymd_opt(2022, 4, 26).expect("Invalid date");
        let events = ALLERGIES_AGGREGATE
            .execute(
                &ALLERGIES_AGGREGATE.init(),
                &AllergiesCommand::CreateAllergies(CreateAllergies {
                    id: "1".to_string(),
                    org_id: "1".to_string(),
                    patient_id: "1".to_string(),
                    allergen: "allergen".to_string(),
                    reaction: "some reactions".to_string(),
                    allergy_severities: AllergySeveritiesType::Mild,
                    input_date,
                    comments: "no comments".to_string(),
                    status: Status::Active,
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
            .fold(ALLERGIES_AGGREGATE.init(), |a, b| {
                ALLERGIES_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.reaction, "some reactions");
    }

    #[test]
    fn allergies_should_be_updated_for_valid_command() {
        let input_date = NaiveDate::from_ymd_opt(2022, 4, 26).expect("Invalid date");
        let events = ALLERGIES_AGGREGATE
            .execute(
                &ALLERGIES_AGGREGATE.init(),
                &AllergiesCommand::UpdateAllergies(UpdateAllergies {
                    org_id: "1".to_string(),
                    patient_id: "1".to_string(),
                    allergen: "allergen".to_string(),
                    reaction: "some reactions".to_string(),
                    allergy_severities: AllergySeveritiesType::Mild,
                    input_date,
                    comments: "no comments".to_string(),
                    id: "1".to_string(),
                    status: Status::Active,
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
            .fold(ALLERGIES_AGGREGATE.init(), |a, b| {
                ALLERGIES_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.reaction, "some reactions");
    }
}
