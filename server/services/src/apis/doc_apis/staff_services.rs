use crate::app_state::AppState;
use crate::doc::staff::command::{add_staff, delete_staff, update_staff};
use crate::get_organization_id::organization_id_fetch;
use crate::method::convention::{self, ErrorData};
use crate::method::methods::gp_methods::staffs::staff::StaffMethods;
use serde_json::Value;
use utils::biscuit::check_role::check_role_exist;

pub async fn process(
    methods: StaffMethods,
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
                (true, StaffMethods::Add(input, _)) => {
                    add_staff(input, state.clone(), org_id, user_id).await
                }
                (true, StaffMethods::Update(input, _)) => {
                    update_staff(input, state.clone(), org_id, user_id).await
                }
                (true, StaffMethods::Delete(input, _)) => {
                    delete_staff(input, state.clone(), org_id, user_id).await
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
