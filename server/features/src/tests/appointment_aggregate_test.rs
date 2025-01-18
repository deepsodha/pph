#[cfg(test)]
mod tests {
    use appointment::{
        domain::{
            appointment_aggregate::APPOINTMENTS_AGGREGATE,
            appointment_commands::{AppointmentCommands, CreateAppointment, UpdateAppointment},
        },
        dto::{appointment_choose_type::ChooseAppointmentType, appointment_visit_type::VisitType},
    };
    use chrono::Utc;
    use cosmo_store_util::aggregate::Aggregate;

    #[test]
    fn appointments_should_be_crated_for_valid_command() {
        let events = APPOINTMENTS_AGGREGATE
            .execute(
                &APPOINTMENTS_AGGREGATE.init(),
                &AppointmentCommands::CreateAppointment(CreateAppointment {
                    id: String::from("1"),
                    org_id: String::from("1"),
                    service_location_id: String::from("1"),
                    patient_id: String::from("1"),
                    doctor_id: String::from("1"),
                    doctor_name: Some(String::from("1")),
                    patient_name: String::from("hey"),
                    visit: VisitType::RegularVisit,
                    date: Utc::now(),
                    appointment_duration: 30,
                    choose_appointment: ChooseAppointmentType::Monthly,
                    note: String::from("1"),
                    room_and_equipment_no: String::from("1"),
                    staff_id: String::from("1"),
                    staff_name: String::from("1"),
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
            .fold(APPOINTMENTS_AGGREGATE.init(), |a, b| {
                APPOINTMENTS_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.org_id, "1");
    }

    #[test]
    fn appointments_should_be_updated_for_valid_command() {
        let events = APPOINTMENTS_AGGREGATE
            .execute(
                &APPOINTMENTS_AGGREGATE.init(),
                &AppointmentCommands::UpdateAppointment(UpdateAppointment {
                    org_id: String::from("1"),
                    id: String::from("1"),
                    service_location_id: String::from("1"),
                    patient_id: String::from("1"),
                    doctor_id: String::from("1"),
                    doctor_name: Some(String::from("1")),
                    patient_name: String::from("hey"),
                    visit: VisitType::RegularVisit,
                    date: Utc::now(),
                    appointment_duration: 30,
                    choose_appointment: ChooseAppointmentType::Monthly,
                    note: String::from("1"),
                    room_and_equipment_no: String::from("1"),
                    staff_id: String::from("1"),
                    staff_name: String::from("1"),
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
            .fold(APPOINTMENTS_AGGREGATE.init(), |a, b| {
                APPOINTMENTS_AGGREGATE.apply(a, b)
            })
            .unwrap();

        assert_eq!(state.org_id, "1");
    }
}
