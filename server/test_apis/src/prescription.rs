#[cfg(test)]
mod tests {
    use crate::helper::{services_helper, URL};
    use chrono::Utc;
    use common::dto::last_updated_input::LastUpdatedInput;
    use log::info;
    use prescription::dto::{
        prescription_add::PrescriptionAdd, prescription_delete::PrescriptionDelete,
        prescription_update::PrescriptionUpdate,
    };
    use services::method::methods::{
        gp_methods::app_methods::AppMethods,
        gp_methods::prescriptions::prescription::PrescriptionMethods,
    };
    use std::time::Instant;

    #[tokio::test]
    pub async fn add_prescription_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Prescription(PrescriptionMethods::Add(
                PrescriptionAdd {
                    patient_name: String::from("1"),
                    presecription_name: String::from("1"),
                    instruction: String::from("1"),
                    date: Utc::now(),
                    drug_name: String::from("1"),
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
                    patient_id: String::from("1"),
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
    pub async fn update_prescription_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Prescription(PrescriptionMethods::Update(
                PrescriptionUpdate {
                    id: String::from("2342342"),
                    input: PrescriptionAdd {
                        patient_name: String::from("2342"),
                        presecription_name: String::from("2342"),
                        instruction: String::from("234324"),
                        date: Utc::now(),
                        drug_name: String::from("2432342"),
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
                        patient_id: String::from("1"),
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
    pub async fn delete_prescription_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Prescription(PrescriptionMethods::Delete(
                PrescriptionDelete {
                    id: String::from("1"),
                    patient_id: String::from("1"),
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
