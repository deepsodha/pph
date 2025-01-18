use crate::app_state::AppState;
use crate::doc::doctors::command::{add_doctors, delete_doctors, update_doctors};
use crate::get_organization_id::organization_id_fetch;
use crate::method::convention::ErrorData;
use crate::method::{convention, methods::gp_methods::doctors::doctor::DoctorMethods};
use serde_json::Value;
use utils::biscuit::check_role::check_role_exist;

pub async fn process(
    methods: DoctorMethods,
    state: &AppState,
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
                (true, DoctorMethods::Add(input, _)) => {
                    add_doctors(input, state.clone(), org_id, user_id).await
                }
                (true, DoctorMethods::Update(input, _)) => {
                    update_doctors(input, state.clone(), org_id, user_id).await
                }
                (true, DoctorMethods::Delete(input, _)) => {
                    delete_doctors(input, state.clone(), org_id, user_id).await
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
