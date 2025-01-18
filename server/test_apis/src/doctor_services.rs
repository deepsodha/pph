#[cfg(test)]
mod tests {

    use std::time::Instant;

    use chrono::Utc;
    use common::dto::{
        address::AddressInput,
        contact::{ContactInput, PhoneNoTypeForContact},
        gender::GenderType,
        gov_info::{self, GovInfoInput},
        last_updated_input::LastUpdatedInput,
        user::UserInput,
    };
    use doctor::dto::{
        doctor_add::DoctorAdd, doctor_delete::DoctorDelete, doctor_type::DoctorType,
        doctor_update::DoctorUpdate,
    };
    use log::info;
    use services::method::methods::gp_methods::{
        app_methods::AppMethods, doctors::doctor::DoctorMethods,
    };

    use crate::helper::{services_helper, URL};

    #[tokio::test]
    pub async fn add_doctors_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Doctors(DoctorMethods::Add(
                DoctorAdd {
                    user: UserInput {
                        first_name: String::from("1"),
                        middle_name: String::from("1"),
                        last_name: String::from("1"),
                        email: String::from("1"),
                        dob: Utc::now(),
                        gender: GenderType::Female,
                        photo_url: String::from("1"),
                    },
                    phone: ContactInput {
                        number: String::from("1"),
                        number_type: PhoneNoTypeForContact::Office,
                    },
                    address: AddressInput {
                        pin_code: String::from("1"),
                        city: String::from("1"),
                        state: String::from("1"),
                        address_line: String::from("1"),
                        country: String::from("1"),
                    },
                    doctor_role: vec![DoctorType::Pharmacist],
                    docter_register_number: String::from("1"),
                    doctor_department: String::from("1"),
                    doctor_speciality: String::from("1"),
                    goverment_info: GovInfoInput {
                        id_no: String::from("1"),
                        id_type: gov_info::GovIdType::DrivingLicense,
                    },
                    password: String::from("123456"),
                    confirm_password: String::from("123456"),
                    emergency: true,
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
    pub async fn update_doctors_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Doctors(DoctorMethods::Update(
                DoctorUpdate {
                    doctor: DoctorAdd {
                        user: UserInput {
                            first_name: String::from("1"),
                            middle_name: String::from("1"),
                            last_name: String::from("1"),
                            email: String::from("1"),
                            dob: Utc::now(),
                            gender: GenderType::Female,
                            photo_url: String::from("1"),
                        },
                        phone: ContactInput {
                            number: String::from("1"),
                            number_type: PhoneNoTypeForContact::Office,
                        },
                        address: AddressInput {
                            pin_code: String::from("1"),
                            city: String::from("1"),
                            state: String::from("1"),
                            address_line: String::from("1"),
                            country: String::from("1"),
                        },
                        doctor_role: vec![DoctorType::Pharmacist],
                        docter_register_number: String::from("1"),
                        doctor_department: String::from("1"),
                        doctor_speciality: String::from("1"),
                        goverment_info: GovInfoInput {
                            id_no: String::from("1"),
                            id_type: gov_info::GovIdType::DrivingLicense,
                        },
                        password: String::from("123456"),
                        confirm_password: String::from("123456"),
                        emergency: true,
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
    pub async fn delete_doctors_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Doctors(DoctorMethods::Delete(
                DoctorDelete {
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
