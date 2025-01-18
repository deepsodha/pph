use super::servicelocation_services::SerivceLocationParams;
use super::{
    appointment_services, doctor_services, organization_services, patient_services,
    presecription_services, servicelocation_services, staff_services, systemadmin_services,
};
use crate::apis::doc_apis::user_services;
use crate::app_state::AppState;
use crate::method::{convention, methods::gp_methods::app_methods::AppMethods};
use serde_json::Value;

pub struct RpcParams {
    pub methods: AppMethods,
    pub org_id: String,
    pub org_name: String,
    pub user_id: String,
    pub user_name: String,
    pub service_location_id: String,
    pub roles: Vec<String>,
    pub feature_access: bool,
}

pub async fn rpc_select(
    app_state: &AppState,
    params: RpcParams,
) -> Result<Value, convention::ErrorData> {
    match params.methods {
        AppMethods::Patients(a) => {
            patient_services::process(
                a,
                app_state,
                params.org_id,
                params.user_id,
                params.feature_access,
                params.roles,
            )
            .await
        }
        AppMethods::Doctors(a) => {
            doctor_services::process(
                a,
                app_state,
                params.org_id,
                params.user_id,
                params.feature_access,
                params.roles,
            )
            .await
        }
        AppMethods::Appointments(a) => {
            appointment_services::process(
                a,
                app_state,
                params.org_id,
                params.user_id,
                params.service_location_id,
                params.feature_access,
                params.roles,
            )
            .await
        }
        AppMethods::Prescription(a) => {
            presecription_services::process(
                a,
                app_state,
                params.org_id,
                params.user_id,
                params.feature_access,
                params.roles,
            )
            .await
        }
        AppMethods::Organizations(a) => {
            organization_services::process(
                a,
                params.roles,
                app_state,
                params.user_id,
                params.user_name,
                params.org_id,
                params.feature_access,
            )
            .await
        }
        AppMethods::SystemAdmin(a) => {
            systemadmin_services::process(
                a,
                app_state,
                params.user_id,
                params.feature_access,
                params.roles,
            )
            .await
        }
        AppMethods::ServiceLocation(a) => {
            servicelocation_services::process(SerivceLocationParams {
                methods: a,
                state: app_state.clone(),
                roles: params.roles,
                org_id: params.org_id,
                org_name: params.org_name,
                user_name: params.user_name,
                user_id: params.user_id,
                feature_access: params.feature_access,
            })
            .await
        }
        AppMethods::Staff(a) => {
            staff_services::process(
                a,
                app_state,
                params.org_id,
                params.user_id,
                params.feature_access,
                params.roles,
            )
            .await
        }
        AppMethods::User(a) => {
            user_services::process(
                a,
                app_state,
                params.org_id,
                params.user_id,
                params.feature_access,
                params.roles,
            )
            .await
        }
    }
}
