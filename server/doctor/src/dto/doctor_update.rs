use super::doctor_add::DoctorAdd;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Default, Clone, Validate, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/DoctorUpdate.ts")]

pub struct DoctorUpdate {
    pub id: String,
    pub doctor: DoctorAdd,
}
