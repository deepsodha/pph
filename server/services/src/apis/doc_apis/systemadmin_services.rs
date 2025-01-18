use crate::app_state::AppState;
use crate::doc::systemadmin::command::{add_systemadmin, delete_systemadmin, update_systemadmin};
use crate::doc::systemadmin::query::get_all_systemadmin;
use crate::method::{convention, methods::gp_methods::system_admin::SystemAdminMethods};
use serde_json::Value;
use utils::biscuit::check_role::check_role_exist;

pub async fn process(
    methods: SystemAdminMethods,
    state: &AppState,
    user_id: String,
    feature_access: bool,
    roles: Vec<String>,
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
            (true, SystemAdminMethods::Add(input, _)) => {
                add_systemadmin(input, state.clone(), user_id).await
            }
            (true, SystemAdminMethods::Update(input, _)) => {
                update_systemadmin(input, state.clone(), user_id).await
            }
            (true, SystemAdminMethods::Delete(input, _)) => {
                delete_systemadmin(input, state.clone(), user_id).await
            }
            (true, SystemAdminMethods::GetAll) => get_all_systemadmin(state.clone()).await,
        }
    } else {
        Err(convention::ErrorData {
            code: -32600,
            message: String::from("Insufficient Role"),
            data: Value::Null,
        })
    }
}
