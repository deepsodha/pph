#[cfg(test)]
mod tests {
    use chrono::Utc;
    use cosmo_store_util::aggregate::Aggregate;
    use patient::domain::problem::{
        problem_aggregate::PROBLEM_AGGREGATE,
        problem_commands::{CreateProblem, ProblemCommand, UpdateProblem},
    };

    #[test]
    fn problems_should_be_crated_for_valid_command() {
        let events = PROBLEM_AGGREGATE
            .execute(
                &PROBLEM_AGGREGATE.init(),
                &ProblemCommand::CreateProblem(CreateProblem {
                    id: "1".to_string(),
                    org_id: "1".to_string(),
                    patient_id: "1".to_string(),
                    status: common::dto::status::Status::Active,
                    issue: "issue".to_string(),
                    icd_10_problem: Some("some problem".to_string()),
                    issue_type: common::dto::types::ProblemTypes::Acute,
                    start_date: chrono::Utc::now(),
                    comment: "no comments".to_string(),
                    end_date: Utc::now(),
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
            .fold(PROBLEM_AGGREGATE.init(), |a, b| {
                PROBLEM_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.comment, "no comments");
    }

    #[test]
    fn problems_should_be_updated_for_valid_command() {
        let events = PROBLEM_AGGREGATE
            .execute(
                &PROBLEM_AGGREGATE.init(),
                &ProblemCommand::UpdateProblem(UpdateProblem {
                    org_id: "1".to_string(),
                    patient_id: "1".to_string(),
                    status: common::dto::status::Status::Active,
                    issue: "issue".to_string(),
                    icd_10_problem: Some("some problem".to_string()),
                    issue_type: common::dto::types::ProblemTypes::Acute,
                    start_date: chrono::Utc::now(),
                    comment: "no comments".to_string(),
                    end_date: Utc::now(),
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
            .fold(PROBLEM_AGGREGATE.init(), |a, b| {
                PROBLEM_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.comment, "no comments");
    }
}
