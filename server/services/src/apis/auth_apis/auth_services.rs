use crate::app_state::AppState;
use crate::auth::auth_services::login_submit;
use crate::method::{convention, methods::auth_methods::authentication::AuthMethods};
use serde_json::Value;

pub async fn process(
    methods: AuthMethods,
    state: &AppState,
) -> Result<Value, convention::ErrorData> {
    match methods {
        AuthMethods::LoginMobile(login_mobile) => login_submit(login_mobile, state.clone()).await,
    }
}
