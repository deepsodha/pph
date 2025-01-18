#[cfg(test)]
mod tests {

    use chrono::Utc;
    use cosmo_store_util::aggregate::Aggregate;
    use patient::domain::immunizations::order::{
        order_aggregate::ORDER_AGGREGATE,
        order_commands::{CreateOrder, OrderCommand},
    };

    #[test]
    fn order_should_be_crated_for_valid_command() {
        let events = ORDER_AGGREGATE
            .execute(
                &ORDER_AGGREGATE.init(),
                &OrderCommand::CreateOrder(CreateOrder {
                    id: "1".to_string(),
                    org_id: "1".to_string(),
                    patient_id: "1".to_string(),
                    vaccine: "allergen".to_string(),
                    types: common::dto::types::Types::Type1,
                    provider: "1".to_string(),
                    ordered: chrono::Utc::now(),
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
            .fold(ORDER_AGGREGATE.init(), |a, b| ORDER_AGGREGATE.apply(a, b))
            .unwrap();

        assert_eq!(state.vaccine, "allergen");
    }
}
