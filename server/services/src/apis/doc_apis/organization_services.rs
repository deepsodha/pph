use crate::app_state::AppState;
use crate::doc::disease_prediction::disease_prediction;
use crate::doc::medicine_prediction::medicine_prediction;
use crate::doc::organization::command::{
    add_organization, delete_organization, select_organization, update_organization,
};
use crate::doc::organization::query::{
    drug_handler, get_all_organization, get_all_pincode_city, get_all_pincode_handler,
    get_all_vaccine_handler, get_by_id_organization, sync_handler,
};
use crate::method::convention;
use crate::method::methods::gp_methods::organizations::organization::OrganizationMethods;
use serde_json::Value;
use utils::biscuit::check_role::check_role_exist;

pub async fn process(
    methods: OrganizationMethods,
    roles: Vec<String>,
    state: &AppState,
    user_id: String,
    user_name: String,
    org_id: String,
    feature_access: bool,
) -> Result<Value, convention::ErrorData> {
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
            (true, OrganizationMethods::Add(input, _)) => {
                add_organization(input, state.clone(), user_id).await
            }
            (true, OrganizationMethods::Update(input, _)) => {
                update_organization(input, state.clone(), user_id).await
            }
            (true, OrganizationMethods::Delete(input, _)) => {
                delete_organization(input, state.clone(), user_id).await
            }
            (true, OrganizationMethods::Get(input, _)) => {
                get_by_id_organization(input, state.clone()).await
            }
            (true, OrganizationMethods::MedicinePrediction(input)) => {
                medicine_prediction(input).await
            }
            (true, OrganizationMethods::DiseasePrediction(input)) => {
                disease_prediction(input).await
            }
            (true, OrganizationMethods::GetAll) => get_all_organization(state.clone()).await,
            (true, OrganizationMethods::PinCodes) => get_all_pincode_handler(state.clone()).await,
            (true, OrganizationMethods::Drugs) => drug_handler().await,
            (true, OrganizationMethods::Vaccines) => get_all_vaccine_handler().await,
            (true, OrganizationMethods::Select(input, _)) => {
                select_organization(input, roles, state.clone(), user_id, user_name).await
            }
            (true, OrganizationMethods::Location(input, _)) => {
                get_all_pincode_city(input, state.clone()).await
            }
            (true, OrganizationMethods::Sync(input, _)) => {
                sync_handler(input, org_id, state.clone()).await
            }
        }
    } else {
        Err(convention::ErrorData {
            code: -32600,
            message: String::from("Insufficient Role"),
            data: Value::Null,
        })
    }
}
