use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::appointment_add::AppointmentAdd;

#[derive(Deserialize, Serialize, Debug, Default, Clone, validator::Validate, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/AppointmentUpdate.ts")]
pub struct AppointmentUpdate {
    pub id: String,
    pub appointment: AppointmentAdd,
}
