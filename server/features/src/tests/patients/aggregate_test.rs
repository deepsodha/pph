#[cfg(test)]
mod tests {

    use chrono::Utc;
    use common::dto::{
        address::AddressInput,
        contact::{ContactInput, PhoneNoTypeForContact},
        gender::GenderType,
        gov_info::{GovIdType, GovInfoInput},
        user::UserInput,
    };
    use cosmo_store_util::aggregate::Aggregate;
    use patient::domain::{
        patient_aggregate::PATIENT_AGGREGATE,
        patient_commands::{CreatePatient, DeletePatient, PatientCommand},
        patient_domain::PatientState,
    };

    #[test]
    fn patient_should_be_created_for_valid_command() {
        let events = PATIENT_AGGREGATE
            .execute(
                &PATIENT_AGGREGATE.init(),
                &PatientCommand::CreatePatient(Box::new(CreatePatient {
                    org_id: "1".to_string(),
                    user: UserInput {
                        first_name: "deep".to_string(),
                        middle_name: "deep".to_string(),
                        last_name: "sodha".to_string(),
                        dob: Utc::now(),
                        email: "infordeep@gmail.com".to_string(),
                        gender: GenderType::Male,
                        photo_url: "https://www.google.com".to_string(),
                    },
                    government_info: GovInfoInput {
                        id_no: "123456789012".to_string(),
                        id_type: GovIdType::AadhaarCard,
                    },
                    phone: ContactInput {
                        number: "9876542310".to_string(),
                        number_type: PhoneNoTypeForContact::Home,
                    },
                    address: AddressInput {
                        pin_code: "sodha".to_string(),
                        city: "sodha".to_string(),
                        state: "sodha".to_string(),
                        address_line: "sodha".to_string(),
                        country: "sodha".to_string(),
                    },
                    created_by: "deep".to_string(),
                    updated_by: "deep".to_string(),
                    created_at: Utc::now(),
                    last_updated: Utc::now(),
                    id: "1".to_string(),
                })),
            )
            .unwrap();

        assert_eq!(events.len(), 1);

        let state = events
            .iter()
            .fold(PATIENT_AGGREGATE.init(), |a, b| {
                PATIENT_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.org_id, "1".to_string());
    }

    // #[test]
    // fn patient_should_be_updated_for_valid_command() {
    //     let events = PATIENT_AGGREGATE.execute(
    //         &PATIENT_AGGREGATE.init(),
    //         &PatientCommand::UpdatePatient(UpdatePatient {
    //             org_id: "1".to_string(),
    //             id: "1".to_string(),
    //             created_by: "deep".to_string(),
    //             updated_by: "deep".to_string(),
    //             created_at: Utc::now(),
    //             last_updated: Utc::now(),
    //             stream_id: "1".to_string(),
    //             version: 32,
    //             patient: UserDetails {
    //                 user: UserInput {
    //                     first_name: "deep".to_string(),
    //                     middle_name: "deep".to_string(),
    //                     last_name: "sodha".to_string(),
    //                     dob: Utc::now(),
    //                     email: "infordeep@gmail.com".to_string(),
    //                     gender: GenderType::Male,
    //                     photo_url: "https://www.google.com".to_string(),
    //                 },
    //                 government_info: GovInfoInput {
    //                     id_no: "123456789012".to_string(),
    //                     id_type: GovIdType::AadhaarCard,
    //                 },
    //                 phone: ContactInput {
    //                     number: "9876542310".to_string(),
    //                     number_type: PhoneNoTypeForContact::Home,
    //                 },
    //                 address: AddressInput {
    //                     pin_code: "sodha".to_string(),
    //                     city: "sodha".to_string(),
    //                     state: "sodha".to_string(),
    //                     address_line: "sodha".to_string(),
    //                     country: "sodha".to_string(),
    //                 },
    //             },
    //         }),
    //     );

    //     match events {
    //         Ok(events) => {
    //             assert_eq!(events.len(), 1);

    //             let updated_state = events.iter().fold(PATIENT_AGGREGATE.init(), |a, b| {
    //                 PATIENT_AGGREGATE.apply(a, b)
    //             });

    //             assert_eq!(updated_state.unwrap().org_id, "1".to_string());
    //         }
    //         Err(e) => panic!("Failed to update patient: {}", e),
    //     }
    // }

    #[test]
    fn patient_should_be_deleted_for_valid_command() {
        let events = PATIENT_AGGREGATE
            .execute(
                &None,
                &PatientCommand::DeletePatient(DeletePatient {
                    org_id: "1".to_string(),
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
            .fold(
                Some(PatientState {
                    id: "1".to_string(),
                    org_id: "1".to_string(),
                    user: UserInput {
                        first_name: "deep".to_string(),
                        middle_name: "deep".to_string(),
                        last_name: "sodha".to_string(),
                        dob: Utc::now(),
                        email: "infordeep@gmail.com".to_string(),
                        gender: GenderType::Male,
                        photo_url: "https://www.google.com".to_string(),
                    },
                    government_info: GovInfoInput {
                        id_no: "123456789012".to_string(),
                        id_type: GovIdType::AadhaarCard,
                    },
                    phone: ContactInput {
                        number: "9876542310".to_string(),
                        number_type: PhoneNoTypeForContact::Home,
                    },
                    address: AddressInput {
                        pin_code: "sodha".to_string(),
                        city: "sodha".to_string(),
                        state: "sodha".to_string(),
                        address_line: "sodha".to_string(),
                        country: "sodha".to_string(),
                    },
                    created_by: "deep".to_string(),
                    updated_by: "deep".to_string(),
                    created_at: Utc::now(),
                    last_updated: Utc::now(),
                    is_deleted: true,
                }),
                |a, b| PATIENT_AGGREGATE.apply(a, b),
            )
            .unwrap();

        assert_eq!(state.org_id, "1".to_string());
    }
}
