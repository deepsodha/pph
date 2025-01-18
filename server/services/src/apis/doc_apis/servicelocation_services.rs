use crate::app_state::AppState;
use crate::doc::servicelocation::command::{
    add_servicelocation, delete_servicelocation, select_servicelocation, update_servicelocation,
};
use crate::doc::servicelocation::query::get_all_service_location;
use crate::get_organization_id::organization_id_fetch;
use crate::method::convention::{self, ErrorData};
use crate::method::methods::gp_methods::servicelocation::service_location::ServiceLocationMethods;
use serde_json::Value;
use utils::biscuit::check_role::check_role_exist;
pub struct SerivceLocationParams {
    pub methods: ServiceLocationMethods,
    pub state: AppState,
    pub roles: Vec<String>,
    pub org_id: String,
    pub org_name: String,
    pub user_name: String,
    pub user_id: String,
    pub feature_access: bool,
}

pub async fn process(params: SerivceLocationParams) -> Result<Value, convention::ErrorData> {
    let organization_id =
        organization_id_fetch(params.state.clone(), params.org_id.clone()).await?;

    if organization_id {
        if check_role_exist(
            vec![
                "systemadmin".to_string(),
                "superadmin".to_string(),
                "doctor".to_string(),
                "officestaff".to_string(),
            ],
            &params.roles,
        ) {
            match (params.feature_access, params.methods) {
                (false, _) => Err(convention::ErrorData {
                    code: -32600,
                    message: String::from("Access Denied"),
                    data: Value::Null,
                }),
                (true, ServiceLocationMethods::Add(input, _)) => {
                    add_servicelocation(input, params.state.clone(), params.org_id, params.user_id)
                        .await
                }
                (true, ServiceLocationMethods::Update(input, _)) => {
                    update_servicelocation(
                        input,
                        params.state.clone(),
                        params.org_id,
                        params.user_id,
                    )
                    .await
                }
                (true, ServiceLocationMethods::Delete(input, _)) => {
                    delete_servicelocation(
                        input,
                        params.state.clone(),
                        params.org_id,
                        params.user_id,
                    )
                    .await
                }
                (true, ServiceLocationMethods::Select(input, _)) => {
                    select_servicelocation(
                        input,
                        params.roles,
                        params.state.clone(),
                        params.user_id,
                        params.org_id,
                        params.org_name,
                        params.user_name,
                    )
                    .await
                }
                (true, ServiceLocationMethods::GetAll) => {
                    get_all_service_location(params.state.clone(), params.org_id).await
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
