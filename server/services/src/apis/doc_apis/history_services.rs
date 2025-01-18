use crate::{
    app_state::AppState,
    doc::patients::history::{
        family_history::command::{add_familyhistory, delete_familyhistory, update_familyhistory},
        hospitalization::command::{
            add_hospitalization, delete_hospitalization, update_hospitalization,
        },
        implantabledevices::command::{
            add_implantable_devices, delete_implantable_devices, update_implantable_devices,
        },
        obandpregnancy::command::{
            add_ob_and_pregnancy, delete_ob_and_pregnancy, update_ob_and_pregnancy,
        },
        pastmedicalhistory::command::{
            add_past_medical_history, delete_past_medical_history, update_past_medical_history,
        },
        pastsurgical_history::command::{
            add_past_surgical_history, delete_past_surgical_history, update_past_surgical_history,
        },
        social_history::command::{
            add_social_history, delete_social_history, update_social_history,
        },
    },
    get_organization_id::organization_id_fetch,
    method::{
        convention::{self, ErrorData},
        methods::gp_methods::patients::history_page::{
            familyhistory::FamilyHistoryMethods, history::HistoryMethods,
            hospitalization::HospitalizationMethods, implantabledevices::ImplantableDevicesMethods,
            obandpregnancy::OBAndPregnancyMethods, pastmedicalhistory::PastMedicalHistoryMethods,
            pastsurgicalhistory::PastSurgicalHistoryMethods, socialhistory::SocialHistoryMethods,
        },
    },
};
use serde_json::Value;
use utils::biscuit::check_role::check_role_exist;

pub async fn process(
    methods: HistoryMethods,
    state: AppState,
    org_id: String,
    user_id: String,
    feature_access: bool,
    roles: Vec<String>,
) -> Result<Value, convention::ErrorData> {
    let organization_id = organization_id_fetch(state.clone(), org_id.clone()).await?;

    if organization_id {
        if check_role_exist(
            vec![
                "systemadmin".to_string(),
                "superadmin".to_string(),
                "doctor".to_string(),
                "officestaff".to_string(),
            ],
            &roles,
        ) {
            match (feature_access, methods) {
                (false, _) => Err(convention::ErrorData {
                    code: -32600,
                    message: String::from("Access Denied"),
                    data: Value::Null,
                }),

                (true, HistoryMethods::FamilyHistory(FamilyHistoryMethods::Add(input))) => {
                    add_familyhistory(input, state.clone(), org_id, user_id).await
                }
                (true, HistoryMethods::FamilyHistory(FamilyHistoryMethods::Update(input))) => {
                    update_familyhistory(input, state.clone(), org_id, user_id).await
                }
                (true, HistoryMethods::FamilyHistory(FamilyHistoryMethods::Delete(input))) => {
                    delete_familyhistory(input, state.clone(), org_id, user_id).await
                }
                (true, HistoryMethods::Hospitalization(HospitalizationMethods::Add(input))) => {
                    add_hospitalization(input, state.clone(), org_id, user_id).await
                }
                (true, HistoryMethods::Hospitalization(HospitalizationMethods::Update(input))) => {
                    update_hospitalization(input, state.clone(), org_id, user_id).await
                }
                (true, HistoryMethods::Hospitalization(HospitalizationMethods::Delete(input))) => {
                    delete_hospitalization(input, state.clone(), org_id, user_id).await
                }
                (
                    true,
                    HistoryMethods::ImplantableDevices(ImplantableDevicesMethods::Add(input)),
                ) => add_implantable_devices(input, state.clone(), org_id, user_id).await,
                (
                    true,
                    HistoryMethods::ImplantableDevices(ImplantableDevicesMethods::Update(input)),
                ) => update_implantable_devices(input, state.clone(), org_id, user_id).await,
                (
                    true,
                    HistoryMethods::ImplantableDevices(ImplantableDevicesMethods::Delete(input)),
                ) => delete_implantable_devices(input, state.clone(), org_id, user_id).await,
                (true, HistoryMethods::ObAndPregnancy(OBAndPregnancyMethods::Add(input))) => {
                    add_ob_and_pregnancy(input, state.clone(), org_id, user_id).await
                }
                (true, HistoryMethods::ObAndPregnancy(OBAndPregnancyMethods::Update(input))) => {
                    update_ob_and_pregnancy(input, state.clone(), org_id, user_id).await
                }
                (true, HistoryMethods::ObAndPregnancy(OBAndPregnancyMethods::Delete(input))) => {
                    delete_ob_and_pregnancy(input, state.clone(), org_id, user_id).await
                }
                (true, HistoryMethods::PastMedicalHistory(boxed_method)) => match *boxed_method {
                    PastMedicalHistoryMethods::Add(input) => {
                        add_past_medical_history(input, state.clone(), org_id, user_id).await
                    }
                    PastMedicalHistoryMethods::Update(input) => {
                        update_past_medical_history(input, state.clone(), org_id, user_id).await
                    }
                    PastMedicalHistoryMethods::Delete(input) => {
                        delete_past_medical_history(input, state.clone(), org_id, user_id).await
                    }
                },
                (
                    true,
                    HistoryMethods::PastSurgicalHistory(PastSurgicalHistoryMethods::Add(input)),
                ) => add_past_surgical_history(input, state.clone(), org_id, user_id).await,
                (
                    true,
                    HistoryMethods::PastSurgicalHistory(PastSurgicalHistoryMethods::Update(input)),
                ) => update_past_surgical_history(input, state.clone(), org_id, user_id).await,
                (
                    true,
                    HistoryMethods::PastSurgicalHistory(PastSurgicalHistoryMethods::Delete(input)),
                ) => delete_past_surgical_history(input, state.clone(), org_id, user_id).await,
                (true, HistoryMethods::SocialHistory(SocialHistoryMethods::Add(input))) => {
                    add_social_history(input, state.clone(), org_id, user_id).await
                }
                (true, HistoryMethods::SocialHistory(SocialHistoryMethods::Update(input))) => {
                    update_social_history(input, state.clone(), org_id, user_id).await
                }
                (true, HistoryMethods::SocialHistory(SocialHistoryMethods::Delete(input))) => {
                    delete_social_history(input, state.clone(), org_id, user_id).await
                }
            }
        } else {
            Err(convention::ErrorData {
                code: -32600,
                message: String::from("Insufficient Role"),
                data: Value::Null,
            })
        }
    } else {
        Err(ErrorData {
            message: String::from("ORGANIZATION_NOT_FOUND"),
            data: Value::Null,
            code: -32600,
        })
    }
}
