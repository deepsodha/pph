#[cfg(test)]
mod tests {

    use chrono::Utc;
    use cosmo_store_util::aggregate::Aggregate;
    use patient::domain::immunizations::notadministered::{
        notadministered_aggregate::NOT_ADMINISTER_AGGREGATE,
        notadministered_commands::{
            CreateNotAdministered, NotAdministerCommand, UpdateNotAdministered,
        },
    };

    #[test]
    fn not_administer_should_be_crated_for_valid_command() {
        let events = NOT_ADMINISTER_AGGREGATE
            .execute(
                &NOT_ADMINISTER_AGGREGATE.init(),
                &NotAdministerCommand::CreateNotAdministered(CreateNotAdministered {
                    id: "1".to_string(),
                    org_id: "1".to_string(),
                    patient_id: "1".to_string(),
                    vaccine: "allergen".to_string(),
                    types: common::dto::types::Types::Type1,
                    recorded: chrono::Utc::now(),
                    reason_for_non_administration: "allergen".to_string(),
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
            .fold(NOT_ADMINISTER_AGGREGATE.init(), |a, b| {
                NOT_ADMINISTER_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.comments, "no comments");
    }

    #[test]
    fn not_administer_should_be_updated_for_valid_command() {
        let events = NOT_ADMINISTER_AGGREGATE
            .execute(
                &NOT_ADMINISTER_AGGREGATE.init(),
                &NotAdministerCommand::UpdateNotAdministered(UpdateNotAdministered {
                    org_id: "1".to_string(),
                    patient_id: "1".to_string(),
                    vaccine: "allergen".to_string(),
                    types: common::dto::types::Types::Type1,
                    recorded: chrono::Utc::now(),
                    reason_for_non_administration: "allergen".to_string(),
                    comments: "no comments".to_string(),
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
            .fold(NOT_ADMINISTER_AGGREGATE.init(), |a, b| {
                NOT_ADMINISTER_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.comments, "no comments");
    }
}
