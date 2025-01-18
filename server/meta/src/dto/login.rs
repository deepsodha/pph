use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "../../bindings/AuthLogin.ts")]
pub struct AuthLogin {
    pub mobile_number: String,
    pub password: String,
}
