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
        status::Status,
        types::{ProblemTypes, Types},
        user::UserInput,
    };
    use doctor::dto::doctor_type::DoctorType;
    use log::info;
    use patient::dto::{
        allergies::{
            allergy_add::AllergyAdd, allergy_delete::AllergyDelete,
            allergy_severities::AllergySeveritiesType, allergy_update::AllergyUpdate,
        },
        history::{
            family_history::{
                familyhistory_add::FamilyHistoryAdd, familyhistory_delete::FamilyHistoryDelete,
                familyhistory_update::FamilyHistoryUpdate,
            },
            health_status::HealthStauts,
            hospitalization::{
                hospitalization_add::HospitalizationAdd,
                hospitalization_delete::HospitalizationDelete,
                hospitalization_update::HospitalizationUpdate,
            },
            implantable_devices::{
                implantabledevices_add::ImplantableDevicesAdd,
                implantabledevices_delete::ImplantableDevicesDelete,
                implantabledevices_update::ImplantableDevicesUpdate,
            },
            ob_and_pregnancy::{
                obandpregnancy_add::OBandPregnancyAdd, obandpregnancy_delete::OBandPregnancyDelete,
                obandpregnancy_update::OBandPregnancyUpdate,
            },
            past_medical_history::{
                pastmedical_add::PastMedicalHistoryAdd,
                pastmedical_delete::PastMedicalHistoryDelete,
                pastmedical_update::PastMedicalHistoryUpdate,
            },
            past_surgical_history::{
                pastsurgicalhistory_add::PastSurgicalHistoryAdd,
                pastsurgicalhistory_delete::PastSurgicalHistoryDelete,
                pastsurgicalhistory_update::PastSurgicalHistoryUpdate,
            },
            social_history::{
                socialhistory_add::SocialHistoryAdd, socialhistory_delete::SocialHistoryDelete,
                socialhistory_update::SocialHistoryUpdate,
            },
        },
        immunization::{
            add_historical_types::{HistoricalAdd, HistoricalDelete, HistoricalUpdate},
            administer_types::{self, AdministerAdd, AdministerDelete, AdministerUpdate, Location},
            not_administered_types::{
                NotAdministeredAdd, NotAdministeredDelete, NotAdministeredUpdate,
            },
            order_types::{OrderAdd, OrderDelete, OrderUpdate},
        },
        medications::{
            medication_add::MedicationsAdd, medication_delete::MedicationDelete,
            medication_update::MedicationUpdate,
        },
        patient_types::{
            patient_add::PatientAdd, patient_delete::PatientDelete, patient_update::PatientUpdate,
        },
        problems::{
            problem_add::ProblemsAdd, problem_delete::ProblemsDelete,
            problem_update::ProblemsUpdate,
        },
        vital::{vital_add::VitalsAdd, vital_delete::VitalsDelete, vital_update::VitalsUpdate},
    };
    use services::method::methods::gp_methods::patients::{
        history_page::{
            familyhistory::FamilyHistoryMethods, history::HistoryMethods,
            hospitalization::HospitalizationMethods, implantabledevices::ImplantableDevicesMethods,
            obandpregnancy::OBAndPregnancyMethods, pastmedicalhistory::PastMedicalHistoryMethods,
            pastsurgicalhistory::PastSurgicalHistoryMethods, socialhistory::SocialHistoryMethods,
        },
        immunization::{
            addhistorical::AddhisitoricalMethods, addminister::AddministerMethods,
            notadminister::NotaddministerMethods,
        },
    };
    use services::method::methods::gp_methods::{
        app_methods::AppMethods,
        patients::{
            allergy::AllergyMethods, immunization::order::OrderMethods,
            medications::MedicationsMethods, patient::PatientsMethods, problem::ProblemsMethods,
            vitals::VitalsMethods,
        },
    };
    use std::time::Instant;

    #[tokio::test]
    pub async fn add_patient_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Add(PatientAdd {
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
            })),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn update_patients_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Update(PatientUpdate {
                id: String::from("1"),
                input: PatientAdd {
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
                },
            })),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn delete_patients_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Delete(PatientDelete {
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
            })),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn add_historical_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::AddHistorical(AddhisitoricalMethods::Add(
                HistoricalAdd {
                    patient_id: String::from("1"),
                    comments: String::from("1"),
                    vaccine: String::from("1"),
                    types: String::from("1"),
                    date: Utc::now(),
                    number_in_series: String::from("1"),
                    source_of_information: String::from("1"),
                    provider: DoctorType::Cardiologist,
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
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn update_historical_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::AddHistorical(
                AddhisitoricalMethods::Update(
                    HistoricalUpdate {
                        id: String::from("1"),
                        input: HistoricalAdd {
                            patient_id: String::from("1"),
                            comments: String::from("1"),
                            vaccine: String::from("1"),
                            types: String::from("1"),
                            date: Utc::now(),
                            number_in_series: String::from("1"),
                            source_of_information: String::from("1"),
                            provider: DoctorType::Cardiologist,
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
                    },
                    None,
                ),
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
    pub async fn delete_historical_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::AddHistorical(
                AddhisitoricalMethods::Delete(
                    HistoricalDelete {
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
                ),
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
    pub async fn add_administer_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Addminister(AddministerMethods::Add(
                AdministerAdd {
                    patient_id: String::from("1"),
                    comments: String::from("1"),
                    vaccine: String::from("1"),
                    types: Types::Type1,
                    brand: administer_types::Brand::Brand1,
                    generic: String::from("1"),
                    ordered: Utc::now(),
                    recorded: Utc::now(),
                    dose: String::from("1"),
                    site: String::from("1"),
                    number_of_serial: 2,
                    expiration: Utc::now(),
                    consent_obtain: String::from("1"),
                    administrated_by: String::from("1"),
                    clinic_location: Location::Home,
                    vis_date: Utc::now(),
                    vfs_financial_class: String::from("1"),
                    lot: 10,
                    provider: DoctorType::Cardiologist,
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
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn update_administer_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Addminister(AddministerMethods::Update(
                AdministerUpdate {
                    id: String::from("1"),
                    input: AdministerAdd {
                        patient_id: String::from("1"),
                        comments: String::from("1"),
                        vaccine: String::from("1"),
                        types: Types::Type1,
                        brand: administer_types::Brand::Brand1,
                        generic: String::from("1"),
                        ordered: Utc::now(),
                        recorded: Utc::now(),
                        dose: String::from("1"),
                        site: String::from("1"),
                        number_of_serial: 2,
                        expiration: Utc::now(),
                        consent_obtain: String::from("1"),
                        administrated_by: String::from("1"),
                        clinic_location: Location::Home,
                        vis_date: Utc::now(),
                        vfs_financial_class: String::from("1"),
                        lot: 10,
                        provider: DoctorType::Cardiologist,
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
                },
                None,
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn delete_administer_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Addminister(AddministerMethods::Delete(
                AdministerDelete {
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
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn add_not_administer_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Notaddminister(NotaddministerMethods::Add(
                NotAdministeredAdd {
                    patient_id: String::from("1"),
                    comments: String::from("1"),
                    vaccine: String::from("1"),
                    types: Types::Type1,
                    recorded: Utc::now(),
                    reason_for_non_administration: String::from("1"),
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
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn update_not_administer_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Notaddminister(
                NotaddministerMethods::Update(
                    NotAdministeredUpdate {
                        input: NotAdministeredAdd {
                            patient_id: String::from("1"),
                            comments: String::from("1"),
                            vaccine: String::from("1"),
                            types: Types::Type1,
                            recorded: Utc::now(),
                            reason_for_non_administration: String::from("1"),
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
                ),
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
    pub async fn delete_not_administer_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Notaddminister(
                NotaddministerMethods::Delete(
                    NotAdministeredDelete {
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
                ),
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
    pub async fn add_allergies() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Allergy(AllergyMethods::Add(
                AllergyAdd {
                    patient_id: String::from("1"),
                    allergen: String::from("1"),
                    reaction: String::from("1"),
                    allergy_severities: AllergySeveritiesType::Mild,
                    input_date: Utc::now(),
                    status: true,
                    comments: String::from("1"),
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
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn update_allergies_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Allergy(AllergyMethods::Update(
                AllergyUpdate {
                    id: String::from("1"),
                    input: AllergyAdd {
                        patient_id: String::from("1"),
                        allergen: String::from("1"),
                        reaction: String::from("1"),
                        allergy_severities: AllergySeveritiesType::Mild,
                        input_date: Utc::now(),
                        status: true,
                        comments: String::from("1"),
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
                },
                None,
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn delete_allergies_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Allergy(AllergyMethods::Delete(
                AllergyDelete {
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
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn add_medication_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Medications(MedicationsMethods::Add(
                MedicationsAdd {
                    patient_id: String::from("1"),
                    status: Status::Active,
                    drug: Some(String::from("1")),
                    instruction: Some(String::from("1")),
                    comments: String::from("1"),
                    patient_name: String::from("1"),
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
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn update_medications_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Medications(MedicationsMethods::Update(
                MedicationUpdate {
                    id: String::from("1"),
                    input: MedicationsAdd {
                        patient_id: String::from("1"),
                        patient_name: String::from("1"),
                        status: Status::Active,
                        drug: Some(String::from("1")),
                        instruction: Some(String::from("1")),
                        comments: String::from("1"),
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
                },
                None,
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn delete_medications_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Medications(MedicationsMethods::Delete(
                MedicationDelete {
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
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn add_problem_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Problems(ProblemsMethods::Add(
                ProblemsAdd {
                    patient_id: String::from("1"),
                    status: Status::Active,
                    issue: String::from("1"),
                    icd_10_problem: Some(String::from("1")),
                    issue_type: ProblemTypes::Acute,
                    start_date: Utc::now(),
                    end_date: Utc::now(),
                    comment: String::from("1"),
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
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn update_problems_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Problems(ProblemsMethods::Update(
                ProblemsUpdate {
                    id: String::from("1"),
                    input: ProblemsAdd {
                        patient_id: String::from("1"),
                        status: Status::Active,
                        issue: String::from("1"),
                        icd_10_problem: Some(String::from("1")),
                        issue_type: ProblemTypes::Acute,
                        start_date: Utc::now(),
                        end_date: Utc::now(),
                        comment: String::from("1"),
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
                },
                None,
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn delete_problems_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Problems(ProblemsMethods::Delete(
                ProblemsDelete {
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
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn add_vitals_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Vitals(VitalsMethods::Add(
                VitalsAdd {
                    patient_id: String::from("1"),
                    date_time: Some(Utc::now()),
                    blood_pressure: Some(20),
                    heart_rate: Some(23),
                    height: Some(20),
                    weight: Some(66),
                    temperature: Some(66),
                    bmi: Some(66),
                    comments: Some(String::from("1")),
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
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn update_vitals_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Vitals(VitalsMethods::Update(
                VitalsUpdate {
                    id: String::from("1"),
                    input: VitalsAdd {
                        patient_id: String::from("1"),

                        date_time: Some(Utc::now()),
                        blood_pressure: Some(20),
                        heart_rate: Some(66),
                        height: Some(20),
                        weight: Some(66),
                        temperature: Some(66),
                        bmi: Some(66),
                        comments: Some(String::from("1")),
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
                },
                None,
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn delete_vitals_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Vitals(VitalsMethods::Delete(
                VitalsDelete {
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
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn add_order_new_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Order(OrderMethods::Add(
                OrderAdd {
                    patient_id: String::from("1"),
                    vaccine: String::from("1"),
                    types: Types::Type1,
                    ordered: Utc::now(),
                    provider: DoctorType::Cardiologist,
                    comments: String::from("1"),
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
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn update_order_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Order(OrderMethods::Update(
                OrderUpdate {
                    id: String::from("1"),
                    input: OrderAdd {
                        patient_id: String::from("1"),
                        vaccine: String::from("1"),
                        types: Types::Type1,
                        ordered: Utc::now(),
                        comments: String::from("1"),
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
                        provider: DoctorType::Cardiologist,
                    },
                },
                None,
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn delete_orders_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::Order(OrderMethods::Delete(
                OrderDelete {
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
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn add_familyhistory_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(HistoryMethods::FamilyHistory(
                FamilyHistoryMethods::Add(FamilyHistoryAdd {
                    patient_id: String::from("1"),
                    family_member: String::from("1"),
                    health_status: HealthStauts::Alive,
                    general: vec![String::from("1")].into(),
                    cancer: vec![String::from("1")].into(),
                    comments: vec![String::from("1")].into(),
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
                }),
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn update_familyhistory_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(HistoryMethods::FamilyHistory(
                FamilyHistoryMethods::Update(FamilyHistoryUpdate {
                    id: String::from("1"),
                    input: FamilyHistoryAdd {
                        patient_id: String::from("1"),
                        family_member: String::from("1"),
                        health_status: HealthStauts::Alive,
                        general: vec![String::from("1")].into(),
                        cancer: vec![String::from("1")].into(),
                        comments: vec![String::from("1")].into(),
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
                }),
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn delete_familyhistory_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(HistoryMethods::FamilyHistory(
                FamilyHistoryMethods::Delete(FamilyHistoryDelete {
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
                }),
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn add_hospitalization_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(HistoryMethods::Hospitalization(
                HospitalizationMethods::Add(HospitalizationAdd {
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
                    admission_date: Utc::now(),
                    related_to: Some(String::from("1")),
                    status: Some(String::from("1")),
                    length_of_stay: Some(12),
                    procedure: Some(String::from("1")),
                    comments: Some(String::from("1")),
                }),
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn update_hospitalization_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(HistoryMethods::Hospitalization(
                HospitalizationMethods::Update(HospitalizationUpdate {
                    id: String::from("1"),
                    input: HospitalizationAdd {
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
                        admission_date: Utc::now(),
                        related_to: Some(String::from("1")),
                        status: Some(String::from("1")),
                        length_of_stay: Some(12),
                        procedure: Some(String::from("1")),
                        comments: Some(String::from("1")),
                    },
                }),
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn delete_hospitalization_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(HistoryMethods::Hospitalization(
                HospitalizationMethods::Delete(HospitalizationDelete {
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
                }),
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn add_implantabledevices_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(
                HistoryMethods::ImplantableDevices(ImplantableDevicesMethods::Add(
                    ImplantableDevicesAdd {
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
                        status: Some(String::from("1")),
                        comments: Some(String::from("1")),
                        udi: Some(String::from("1")),
                        udi_unknown: Some(true),
                    },
                )),
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
    pub async fn update_implantabledevices_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(
                HistoryMethods::ImplantableDevices(ImplantableDevicesMethods::Update(
                    ImplantableDevicesUpdate {
                        id: String::from("1"),
                        input: ImplantableDevicesAdd {
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
                            status: Some(String::from("1")),
                            comments: Some(String::from("1")),
                            udi: Some(String::from("1")),
                            udi_unknown: Some(true),
                        },
                    },
                )),
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
    pub async fn delete_implantabledevices_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(
                HistoryMethods::ImplantableDevices(ImplantableDevicesMethods::Delete(
                    ImplantableDevicesDelete {
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
                )),
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
    pub async fn add_obandpregnanacy_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(HistoryMethods::ObAndPregnancy(
                OBAndPregnancyMethods::Add(OBandPregnancyAdd {
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
                    age_onset_of_menses: 1,
                    age_at_menopause: 1,
                    comments_ob: Some(String::from("1")),
                    total_pregnancy: Some(1),
                    full_term: Some(1),
                    pre_term: Some(1),
                    miscarriages: Some(1),
                    living: Some(1),
                    comments_pregnancy: Some(String::from("1")),
                }),
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn update_obandpregnanacy_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(HistoryMethods::ObAndPregnancy(
                OBAndPregnancyMethods::Update(OBandPregnancyUpdate {
                    id: String::from("1"),
                    input: OBandPregnancyAdd {
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
                        age_onset_of_menses: 1,
                        age_at_menopause: 1,
                        comments_ob: Some(String::from("1")),
                        total_pregnancy: Some(1),
                        full_term: Some(1),
                        pre_term: Some(1),
                        miscarriages: Some(1),
                        living: Some(1),
                        comments_pregnancy: Some(String::from("1")),
                    },
                }),
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn delete_obandpregnanacy_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(HistoryMethods::ObAndPregnancy(
                OBAndPregnancyMethods::Delete(OBandPregnancyDelete {
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
                    id: String::from("1"),
                }),
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn add_pastmedical_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(
                HistoryMethods::PastMedicalHistory(Box::new(PastMedicalHistoryMethods::Add(
                    PastMedicalHistoryAdd {
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
                        blood_type: Some(String::from("1")),
                        head: vec![String::from("1")].into(),
                        respiratory: vec![String::from("1")].into(),
                        musculoskeletal: vec![String::from("1")].into(),
                        endocrine: vec![String::from("1")].into(),
                        eyes: vec![String::from("1")].into(),
                        gastrointestinal: vec![String::from("1")].into(),
                        skin: vec![String::from("1")].into(),
                        ears: vec![String::from("1")].into(),
                        noses: vec![String::from("1")].into(),
                        neurological: vec![String::from("1")].into(),
                        heme: vec![String::from("1")].into(),
                        mouth: vec![String::from("1")].into(),
                        infectious: vec![String::from("1")].into(),
                        cardiovascular: vec![String::from("1")].into(),
                        genitourinary: vec![String::from("1")].into(),
                        psychiatric: vec![String::from("1")].into(),
                        comments: Some(String::from("1")),
                    },
                ))),
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
    pub async fn update_pastmedical_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(
                HistoryMethods::PastMedicalHistory(Box::new(PastMedicalHistoryMethods::Update(
                    PastMedicalHistoryUpdate {
                        id: String::from("1"),
                        input: PastMedicalHistoryAdd {
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
                            blood_type: Some(String::from("1")),
                            head: vec![String::from("1")].into(),
                            respiratory: vec![String::from("1")].into(),
                            musculoskeletal: vec![String::from("1")].into(),
                            endocrine: vec![String::from("1")].into(),
                            eyes: vec![String::from("1")].into(),
                            gastrointestinal: vec![String::from("1")].into(),
                            skin: vec![String::from("1")].into(),
                            ears: vec![String::from("1")].into(),
                            noses: vec![String::from("1")].into(),
                            neurological: vec![String::from("1")].into(),
                            heme: vec![String::from("1")].into(),
                            mouth: vec![String::from("1")].into(),
                            infectious: vec![String::from("1")].into(),
                            cardiovascular: vec![String::from("1")].into(),
                            genitourinary: vec![String::from("1")].into(),
                            psychiatric: vec![String::from("1")].into(),
                            comments: Some(String::from("1")),
                        },
                    },
                ))),
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
    pub async fn delete_pastmedical_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(
                HistoryMethods::PastMedicalHistory(Box::new(PastMedicalHistoryMethods::Delete(
                    PastMedicalHistoryDelete {
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
                        id: String::from("1"),
                    },
                ))),
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
    pub async fn add_pastsurgical_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(
                HistoryMethods::PastSurgicalHistory(PastSurgicalHistoryMethods::Add(
                    PastSurgicalHistoryAdd {
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
                        comments: Some(String::from("1")),
                        common_surgeries: vec![String::from("1")].into(),
                    },
                )),
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
    pub async fn update_pastsurgical_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(
                HistoryMethods::PastSurgicalHistory(PastSurgicalHistoryMethods::Update(
                    PastSurgicalHistoryUpdate {
                        id: String::from("1"),
                        input: PastSurgicalHistoryAdd {
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
                            comments: Some(String::from("1")),
                            common_surgeries: vec![String::from("1")].into(),
                        },
                    },
                )),
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
    pub async fn delete_pastsurgical_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(
                HistoryMethods::PastSurgicalHistory(PastSurgicalHistoryMethods::Delete(
                    PastSurgicalHistoryDelete {
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
                        id: String::from("1"),
                    },
                )),
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
    pub async fn add_socialhistory_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(HistoryMethods::SocialHistory(
                SocialHistoryMethods::Add(SocialHistoryAdd {
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
                    comments: Some(String::from("1")),
                    birth_gender: GenderType::Male,
                    tobacco: vec![String::from("1")].into(),
                    alcohol: vec![String::from("1")].into(),
                    cardiovascular: vec![String::from("1")].into(),
                    sexual_activity: vec![String::from("1")].into(),
                    drug_abuse: vec![String::from("1")].into(),
                    safety: vec![String::from("1")].into(),
                }),
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn update_socialhistory_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(HistoryMethods::SocialHistory(
                SocialHistoryMethods::Update(SocialHistoryUpdate {
                    id: String::from("1"),
                    input: SocialHistoryAdd {
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
                        comments: Some(String::from("1")),
                        birth_gender: GenderType::Male,
                        tobacco: vec![String::from("1")].into(),
                        alcohol: vec![String::from("1")].into(),
                        cardiovascular: vec![String::from("1")].into(),
                        sexual_activity: vec![String::from("1")].into(),
                        drug_abuse: vec![String::from("1")].into(),
                        safety: vec![String::from("1")].into(),
                    },
                }),
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    pub async fn delete_socialhistory_check() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        let res: Result<String, String> = services_helper::<String>(
            AppMethods::Patients(PatientsMethods::History(HistoryMethods::SocialHistory(
                SocialHistoryMethods::Delete(SocialHistoryDelete {
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
                    id: String::from("1"),
                }),
            ))),
            URL,
        )
        .await;
        let duration = start.elapsed();
        info!("duration: {:?}", duration);
        assert!(res.is_ok());
        Ok(())
    }
}
