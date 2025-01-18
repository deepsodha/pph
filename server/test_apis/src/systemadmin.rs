#[cfg(test)]
mod tests {
    use crate::helper::{services_helper, URL};
    use chrono::Utc;
    use common::dto::{
        address::AddressInput,
        contact::{ContactInput, PhoneNoTypeForContact},
        gender::GenderType,
        gov_info,
        last_updated_input::LastUpdatedInput,
        user::UserInput,
        user_role::UserRole,
    };
    use log::info;
    use services::method::methods::gp_methods::{
        app_methods::AppMethods, system_admin::SystemAdminMethods,
    };
    use std::time::Instant;
    use system_admin::dto::{
        system_admin_add::SystemAdminAdd, system_admin_delete::SystemAdminDelete,
        system_admin_update::SystemAdminUpdate,
    };

    #[tokio::test]
    pub async fn add_systemadmin_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::SystemAdmin(SystemAdminMethods::Add(
                SystemAdminAdd {
                    user: UserInput {
                        first_name: String::from("1"),
                        middle_name: String::from("1"),
                        last_name: String::from("1"),
                        dob: Utc::now(),
                        email: String::from("1"),
                        gender: GenderType::Female,
                        photo_url: String::from("1"),
                    },
                    phone: ContactInput {
                        number: String::from("1"),
                        number_type: PhoneNoTypeForContact::Mobile,
                    },
                    government_info: gov_info::GovInfoInput {
                        id_no: String::from("1"),
                        id_type: gov_info::GovIdType::DrivingLicense,
                    },
                    address: AddressInput {
                        pin_code: String::from("1"),
                        city: String::from("1"),
                        state: String::from("1"),
                        address_line: String::from("1"),
                        country: String::from("1"),
                    },
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
                    roles: vec![UserRole::Patient],
                    password: String::from("1"),
                    confirm_password: String::from("1"),
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
    pub async fn update_systemadmin_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::SystemAdmin(SystemAdminMethods::Update(
                SystemAdminUpdate {
                    id: String::from("1"),
                    input: SystemAdminAdd {
                        user: UserInput {
                            first_name: String::from("1"),
                            middle_name: String::from("1"),
                            last_name: String::from("1"),
                            dob: Utc::now(),
                            email: String::from("1"),
                            gender: GenderType::Female,
                            photo_url: String::from("1"),
                        },
                        phone: ContactInput {
                            number: String::from("1"),
                            number_type: PhoneNoTypeForContact::Mobile,
                        },
                        government_info: gov_info::GovInfoInput {
                            id_no: String::from("1"),
                            id_type: gov_info::GovIdType::DrivingLicense,
                        },
                        address: AddressInput {
                            pin_code: String::from("1"),
                            city: String::from("1"),
                            state: String::from("1"),
                            address_line: String::from("1"),
                            country: String::from("1"),
                        },
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
                        roles: vec![UserRole::Patient],
                        password: String::from("1"),
                        confirm_password: String::from("1"),
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
    pub async fn delete_systemadmin_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::SystemAdmin(SystemAdminMethods::Delete(
                SystemAdminDelete {
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
