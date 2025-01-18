#[cfg(test)]
mod tests {
    use crate::helper::{services_helper, URL};
    use appointment::dto::{
        appointment_add::AppointmentAdd, appointment_delete::AppointmentDelete,
        appointment_update::AppointmentUpdate, choose_appointment_type::ChooseAppointmentType,
        visit_type::VisitType,
    };
    use chrono::Utc;
    use common::dto::last_updated_input::LastUpdatedInput;
    use log::info;
    use services::method::methods::{
        gp_methods::app_methods::AppMethods,
        gp_methods::appointments::appointment::AppointmentMethods,
    };
    use std::time::Instant;

    #[tokio::test]
    pub async fn add_appointments_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Appointments(AppointmentMethods::Add(
                AppointmentAdd {
                    patient_id: String::from("1"),
                    doctor_id: String::from("1"),
                    doctor_name: Some(String::from("1")),
                    patient_name: String::from("1"),
                    visit: VisitType::RegularVisit,
                    date: Utc::now(),
                    appointment_duration: 21,
                    choose_appointment: ChooseAppointmentType::Monthly,
                    note: String::from("1"),
                    room_and_equipment_no: String::from("1"),
                    staff_id: String::from("1"),
                    last_updated_input: LastUpdatedInput {
                        system_admin: Utc::now(),
                        doctors: Utc::now(),
                        patients: Utc::now(),
                        appointments: Utc::now(),
                        prescription: Utc::now(),
                        service_location: Utc::now(),
                        add_historical: Utc::now(),
                        administer: Utc::now(),
                        allergy: Utc::now(),
                        medication: Utc::now(),
                        not_administer: Utc::now(),
                        order: Utc::now(),
                        problems: Utc::now(),
                        vitals: Utc::now(),
                        familyhistory: Utc::now(),
                        hospitalization: Utc::now(),
                        implantabledevices: Utc::now(),
                        obandpregnanacy: Utc::now(),
                        pastmedicalhistory: Utc::now(),
                        pastsurgicalhistory: Utc::now(),
                        socialhistory: Utc::now(),
                        staff: Utc::now(),
                    },
                },
                None,
            )),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn update_appointments_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Appointments(AppointmentMethods::Update(
                AppointmentUpdate {
                    appointment: AppointmentAdd {
                        doctor_name: Some(String::from("1")),
                        patient_name: String::from("1"),
                        visit: VisitType::RegularVisit,
                        date: Utc::now(),
                        appointment_duration: 21,
                        choose_appointment: ChooseAppointmentType::Monthly,
                        note: String::from("1"),
                        room_and_equipment_no: String::from("1"),
                        staff_id: String::from("1"),
                        patient_id: String::from("1"),
                        doctor_id: String::from("1"),
                        last_updated_input: LastUpdatedInput {
                            system_admin: Utc::now(),
                            doctors: Utc::now(),
                            patients: Utc::now(),
                            appointments: Utc::now(),
                            prescription: Utc::now(),
                            service_location: Utc::now(),
                            add_historical: Utc::now(),
                            administer: Utc::now(),
                            allergy: Utc::now(),
                            medication: Utc::now(),
                            not_administer: Utc::now(),
                            order: Utc::now(),
                            problems: Utc::now(),
                            vitals: Utc::now(),
                            familyhistory: Utc::now(),
                            hospitalization: Utc::now(),
                            implantabledevices: Utc::now(),
                            obandpregnanacy: Utc::now(),
                            pastmedicalhistory: Utc::now(),
                            pastsurgicalhistory: Utc::now(),
                            socialhistory: Utc::now(),
                            staff: Utc::now(),
                        },
                    },
                    id: String::from("1"),
                },
                None,
            )),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn delete_appointments_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Appointments(AppointmentMethods::Delete(
                AppointmentDelete {
                    patient_id: String::from("1"),
                    doctor_id: String::from("1"),
                    id: String::from("1"),
                    last_updated_input: LastUpdatedInput {
                        system_admin: Utc::now(),
                        doctors: Utc::now(),
                        patients: Utc::now(),
                        appointments: Utc::now(),
                        prescription: Utc::now(),
                        service_location: Utc::now(),
                        add_historical: Utc::now(),
                        administer: Utc::now(),
                        allergy: Utc::now(),
                        medication: Utc::now(),
                        not_administer: Utc::now(),
                        order: Utc::now(),
                        problems: Utc::now(),
                        vitals: Utc::now(),
                        familyhistory: Utc::now(),
                        hospitalization: Utc::now(),
                        implantabledevices: Utc::now(),
                        obandpregnanacy: Utc::now(),
                        pastmedicalhistory: Utc::now(),
                        pastsurgicalhistory: Utc::now(),
                        socialhistory: Utc::now(),
                        staff: Utc::now(),
                    },
                },
                None,
            )),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }
}
